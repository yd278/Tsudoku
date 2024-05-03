#include "Grid.h"

#include <bitset>
#include <map>
#include <random>
#include <utility>
#include <vector>

#include "solvers/DLX.h"
#include "solvers/allSolvers.h"
#include "util.h"

void Grid::updateStrongLinks() {
    for (auto &row : strongLinks) row.clear();
    FOR_ALL(i) FOR_ALL(j){
        auto &SL = grid[i][j].SL;
        std::fill(SL.begin(),SL.end(),nullptr);
    }
    for (int houseType : {0, 1, 2}) {
        FOR_ALL(houseID) {
            FOR_ALL(target) {
                std::vector<const Cell *> tmp;
                FOR_ALL(i) {
                    auto pos = convert(houseID, i, houseType);
                    auto cell = getCell(pos);

                    if (cell->value != 0) continue;
                    if (cell->candidates[target]) {
                        // tmp.push_back(getCell(pos));
                        tmp.push_back(cell);
                    }
                }
                if (tmp.size() == 2) {
                    strongLinks[target].push_back(
                        std::make_pair(tmp[0], tmp[1]));
                    Cell &a = grid[tmp[0]->x][tmp[0]->y];
                    Cell &b = grid[tmp[1]->x][tmp[1]->y];
                    a.SL[houseType * 9 + target] = &b;
                    b.SL[houseType * 9 + target] = &a;
                }
            }
        }
    }
}
void Grid::checkAndFill(std::string gridPattern) {
    if (gridPattern.length() != 972) {
        throw std::invalid_argument("Invalid Pattern: wrong length");
    }
    // initialize grid

    // sanity check for each cell while filling in
    for (int i = 0; i < 81; i++) {
        Cell &cur = grid[i / 9][i % 9];
        std::string curCell = gridPattern.substr(i * 12, 12);

        // given flag
        if (curCell[0] != '0' && curCell[0] != '1')
            throw std::invalid_argument("Invalid Pattern: invalid given flag");
        cur.given = curCell[0] == '1';

        // value
        if (curCell[1] < '0' || curCell[1] > '9')
            throw std::invalid_argument("Invalid Pattern: invalid value");
        cur.value = curCell[1] - '0';

        // candidates
        if (cur.value == 0) {
            for (int j = 0; j < 9; j++) {
                if (curCell[j + 2] != '0' && curCell[j + 2] != '1')
                    throw std::invalid_argument(
                        "Invalid Pattern: invalid candidates");
                cur.candidates[j] = curCell[j + 2] == '1';
            }
        } else {
            for (int j = 0; j < 9; j++) {
                if (curCell[j + 2] != '0')
                    throw std::invalid_argument(
                        "Invalid Pattern: invalid candidates");
            }
        }

        // answer
        if (curCell[11] < '0' || curCell[11] > '9')
            throw std::invalid_argument("Invalid Pattern: invalid answer");
        cur.ans = curCell[11] - '0';
    }
}
void Grid::updateCandCouldBe() {
    FOR_ALL(x) FOR_ALL(y) {
        if (grid[x][y].given) {
            FOR_ALL(tar) grid[x][y].candCouldBe[tar] = false;
            int val = grid[x][y].value - 1;
            FOR_ALL(row) grid[row][y].candCouldBe[val] = false;
            FOR_ALL(col) grid[x][col].candCouldBe[val] = false;
            int box = (x / 3) * 3 + (y / 3);
            FOR_ALL(index) {
                auto pos = convert(box, index, 2);
                grid[pos.first][pos.second].candCouldBe[val] = false;
            }
        }
    }
}
bool Grid::checkWrongValues() {
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (grid[i][j].value != 0 && grid[i][j].value != grid[i][j].ans)
                return false;
        }
    }
    return true;
}

bool Grid::checkMissingCandidates() {
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (grid[i][j].value == 0) {
                if (!grid[i][j].candidates[grid[i][j].ans - 1]) return false;
            }
        }
    }
    return true;
}

const Cell *Grid::getCell(int houseType, int houseID, int cellID) const {
    return getCell(convert(houseID, cellID, houseType));
}

bool Grid::checkWrongCandidates() {
    for (int houseType = 0; houseType < 3; houseType++) {
        for (int i = 0; i < 9; i++) {
            std::bitset<9> candidatesUnion;
            for (int j = 0; j < 9; j++) {
                auto coordinate = convert(i, j, houseType);
                Cell &cell = grid[coordinate.first][coordinate.second];
                if (grid[i][j].value == 0) {
                    candidatesUnion = candidatesUnion | grid[i][j].candidates;
                } else {
                    if (candidatesUnion[grid[i][j].value - 1]) return false;
                }
            }
        }
    }
    return true;
}
std::string Grid::compress() {
    std::string compressed = "";
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (grid[i][j].given) {
                compressed += grid[i][j].value + '0';
            } else {
                compressed += '0';
            }
        }
    }
    return compressed;
}
void Grid::uniqueness(bool keepIt) {
    std::string compressed = compress();

    std::string res;
    try {
        res = solve(compressed);
    } catch (std::invalid_argument &e) {
        if (std::strcmp(e.what(), "No Solution") == 0)
            throw;
        else
            res = e.what();
        if (keepIt) FOR_ALL(i) FOR_ALL(j) {
                grid[i][j].ans = res[i * 9 + j] - '0';
            }
        throw std::invalid_argument("multiple solutions");
    }
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (!grid[i][j].ans)
                grid[i][j].ans = res[i * 9 + j] - '0';
            else if (grid[i][j].ans != res[i * 9 + j] - '0')
                throw std::invalid_argument(
                    "Invalid Sudoku: provided answer doesn't match");
        }
    }
}

void Grid::updateBiValues() {
    biValues.clear();
    FOR_ALL(i) FOR_ALL(j) biValuesByCands[i][j].clear();
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (grid[i][j].candidates.count() == 2) {
                biValues.push_back(&grid[i][j]);
                std::vector<int> cands;
                FOR_ALL(can)
                if (grid[i][j].candidates[can]) cands.push_back(can);
                biValuesByCands[cands[1]][cands[0]].push_back(&grid[i][j]);
            }
        }
    }
}
void Grid::updateGraph() {
    std::map<std::pair<const Cell *, int>, int> lookup;
    nodes.clear();
    int cnt = 0;
    FOR_ALL(i) FOR_ALL(j) {
        FOR_ALL(cand) {
            if (grid[i][j].candidates[cand]) {
                nodes.push_back(Node{cnt, false, cand, i, j});
                lookup[std::make_pair(&grid[i][j], cand)] = cnt;
                cnt++;
            }
        }
    }
    for (int i = 0; i < cnt; i++) {
        auto &n = nodes[i];
        Node p = Node{i + cnt, true, n.target, n.x, n.y};
        nodes.push_back(std::move(p));
    }
    // build Strong Links;
    // bi-locals
    FOR_ALL(target) {
        for (auto sl : strongLinks[target]) {
            auto fir = sl.first;
            auto sec = sl.second;
            int fn = lookup[std::make_pair(fir, target)];
            int sn = lookup[std::make_pair(sec, target)];
            nodes[fn].edges.push_back(sn + cnt);
            nodes[sn].edges.push_back(fn + cnt);
        }
    }
    // bi-values
    for (auto bv : biValues) {
        int x, y;
        FOR_ALL(i) if (bv->candidates[i]) {
            x = i;
            break;
        }
        for (int i = x + 1; i < 9; i++)
            if (bv->candidates[i]) {
                y = i;
                break;
            }
        int xn = lookup[std::make_pair(bv, x)];
        int yn = lookup[std::make_pair(bv, y)];
        nodes[xn].edges.push_back(yn + cnt);
        nodes[yn].edges.push_back(xn + cnt);
    }

    // week links
    FOR_ALL(i) FOR_ALL(j) {
        const Cell *cell = &grid[i][j];
        FOR_ALL(target) {
            if (!cell->candidates[target]) continue;
            int cur = lookup[std::make_pair(cell, target)] + cnt;
            // same cell:
            FOR_ALL(other_target) {
                if (!cell->candidates[other_target]) continue;
                if (other_target == target) continue;
                int other = lookup[std::make_pair(cell, other_target)];
                nodes[cur].edges.push_back(other);
            }
            // different cell:
            FOR_ALL(oi) FOR_ALL(oj) {
                const Cell *other = &grid[oi][oj];
                if (!sees(cell, other)) continue;
                if (other->candidates[target]) {
                    int oth = lookup[std::make_pair(other, target)];
                    nodes[cur].edges.push_back(oth);
                }
            }
        }
    }
}
Grid::Grid()
    : grid(9, std::vector<Cell>(9)),
      strongLinks(9),
      filled(3, std::vector<std::bitset<9>>(9)),
      biValuesByCands(9, std::vector<std::vector<const Cell *>>(
                             9, std::vector<const Cell *>(9))),
      distr(0, 8) {
    FOR_ALL(i) FOR_ALL(j) {
        grid[i][j].x = i;
        grid[i][j].y = j;
    }

    if (rd.entropy() != 0) {
        seed = rd();
    } else {
        seed = std::chrono::system_clock::now().time_since_epoch().count();
    }
    gen.seed(seed);
}
Grid::Grid(std::string gridPattern) : Grid() {
    try {
        checkAndFill(gridPattern);
        uniqueness();
    } catch (const std::invalid_argument &e) {
        throw;
    }

    if (!checkWrongValues()) throw std::invalid_argument("Wrong cell values");

    if (!checkWrongCandidates())
        throw std::invalid_argument("Wrong candidates");

    if (!checkMissingCandidates())
        throw std::invalid_argument("Missing candidates");

    updateCandCouldBe();
    updateBiValues();
    updateStrongLinks();
    updateGraph();
    updateFilled();
}

Grid::Grid(int difficulty) : Grid() {
    // TODO: generate sudoku of given difficulty
    // NOTE: during nextStep() and execution, if a value is set, elinimate all
    // the same candidates see that.

    int MAX_TRAIL = 100000;
    int cnt = 0;
    while (cnt < MAX_TRAIL) {
        cnt++;
        if (!generateFullBoard()) continue;
        digHoles();

        FOR_ALL(x) FOR_ALL(y) {
            if (grid[x][y].given) continue;
            grid[x][y].candCouldBe.set();
            grid[x][y].value = 0;
        }
        updateCandCouldBe();
        FOR_ALL(i) FOR_ALL(j) {
            grid[i][j].candidates = grid[i][j].candCouldBe;
        }

        updateBiValues();
        updateStrongLinks();
        updateGraph();
        updateFilled();

        int d = checkDifficulty();

        if (d == difficulty) {
            return;
        }
    }
}

std::string Grid::toString() {
    std::string res = "";
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            res += (grid[i][j].given ? grid[i][j].value : 0) + '0';
        }
    }
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            res += grid[i][j].ans + '0';
        }
    }
    return res;
}

const Inst *Grid::nextStep() {
    initInsAndExe();
    for (auto solver : solvers) {
        solver(*this);
        if (!instructions.empty()) break;
    }
    return &instructions;
}

const Cell *Grid::getCell(int x, int y) const { return &grid[x][y]; }

const Cell *Grid::getCell(std::pair<int, int> pos) const {
    return &grid[pos.first][pos.second];
}

void Grid::addExec(const Cell *cell, uint8_t cand) {
    addExec(encodePos(cell), cand);
}

void Grid::updateFilled() {
    for (auto &houses : filled) {
        for (auto &house : houses) {
            house.reset();
        }
    }
    FOR_ALL(i) FOR_ALL(j) {
        if (grid[i][j].value) {
            int box = findBox(i, j);
            int v = grid[i][j].value - 1;
            filled[0][i].set(v);
            filled[1][j].set(v);
            filled[2][box].set(v);
        }
    }
}
std::string Grid::toTestString() {
    std::string res = "";
    FOR_ALL(i) FOR_ALL(j) {
        auto cell = getCell(i, j);
        res += cell->given ? "1" : "0";
        res += cell->value + '0';
        FOR_ALL(cand) res += cell->candidates[cand]?"1":"0";
        res += cell->ans + '0';
    }
    return res;
}
int Grid::checkDifficulty() {
    int maxDifficulty = 0;
    while (!completed()) {
        nextStep();
        if (instructions.empty()) {
            return 4;
        }
        int difficulty = instructions[0] >> 6;
        if (maxDifficulty < difficulty) maxDifficulty = difficulty;
        execute();
    }
    return maxDifficulty;
}

void Grid::execute() {
    if (execution.mode) {
        for (auto exec : execution.executees) {
            int x = exec >> 12;
            int y = (exec >> 8) & 0xf;
            int target = exec & 0xf;
            grid[x][y].value = target + 1;
            grid[x][y].candidates.reset();
            //  auto eliminate candidates
            int box = findBox(x, y);
            FOR_ALL(index) {
                grid[x][index].candidates.reset(target);
                grid[index][y].candidates.reset(target);
                auto pos = convert(box, index, 2);
                grid[pos.first][pos.second].candidates.reset(target);
            }
        }

    } else {
        for (auto exec : execution.executees) {
            int x = exec >> 12;
            int y = (exec >> 8) & 0xf;
            int target = exec & 0xf;
            grid[x][y].candidates[target] = false;
        }
    }
    updateBiValues();
    updateStrongLinks();
    updateGraph();
    updateFilled();
}
bool Grid::completed() {
    FOR_ALL(i) FOR_ALL(j) {
        if (grid[i][j].value == 0) return false;
    }
    return true;
}

bool Grid::generateFullBoard() {
    FOR_ALL(i) FOR_ALL(j) {
        grid[i][j].value = 0;
        grid[i][j].given = false;
        grid[i][j].ans = 0;
        grid[i][j].candidates.reset();
        grid[i][j].candCouldBe.set();
    }
    int hintCnt = 0;

    std::vector<int> perm(81);
    for (int i = 0; i < 81; i++) {
        perm[i] = i;
    }
    std::vector<int> cands(9);
    int flag = false;
    for (int i = 0; i < 9; i++) cands[i] = i;

    std::shuffle(perm.begin(), perm.end(), gen);
    for (auto p : perm) {
        int i = p / 9;
        int j = p % 9;


        std::shuffle(cands.begin(), cands.end(), gen);

        hintCnt++;
        for (auto target : cands) {
            int box = findBox(i, j);
            bool valid = true;
            FOR_ALL(index) {
                if (grid[i][index].value == target + 1) {
                    valid = false;
                    break;
                }
                if (grid[index][j].value == target + 1) {
                    valid = false;
                    break;
                }
                if (getCell(2, box, index)->value == target + 1) {
                    valid = false;
                    break;
                }
            }
            if (!valid) continue;

            grid[i][j].value = target + 1;
            grid[i][j].given = true;
            if (hintCnt >= 17) {
                try {
                    uniqueness(true);
                    FOR_ALL(ti) FOR_ALL(tj) {
                        grid[ti][tj].value = grid[ti][tj].ans;
                        grid[ti][tj].given = true;
                    }
                    return true;
                } catch (const std::invalid_argument &e) {
                    if (std::strcmp(e.what(), "No Solution") == 0) {
                        if (flag) {
                            FOR_ALL(ti) FOR_ALL(tj) {
                                grid[ti][tj].value = grid[ti][tj].ans;
                                grid[ti][tj].given = true;
                            }
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        flag = true;
                        break;
                    }
                }
            }
        }
    }
    return false;
}

void Grid::digHoles() {
    std::vector<int> perm(81);
    for (int i = 0; i < 81; i++) {
        perm[i] = i;
    }
    std::shuffle(perm.begin(), perm.end(), gen);
    for (auto i : perm) {
        int x = i / 9;
        int y = i % 9;

        grid[x][y].given = false;
        grid[x][y].value = 0;
        try {
            uniqueness();

        } catch (const std::invalid_argument &e) {
            grid[x][y].given = true;
            grid[x][y].value = grid[x][y].ans;
        }
    }
}