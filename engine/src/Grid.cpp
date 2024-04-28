#include "Grid.h"

#include <bitset>
#include <map>
#include <utility>
#include <vector>

#include "solvers/DLX.h"
#include "util.h"

void Grid::updateStrongLinks() {
    for (auto &row : strongLinks) row.clear();
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
    grid.resize(9);
    for (auto &row : grid) row.resize(9);

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

void Grid::uniqueness() {
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

    std::string res;
    try {
        // will throw exception if there are more than one solution
        res = solve(compressed);
    } catch (std::invalid_argument &e) {
        throw;
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
    biValuesByCands.clear();
    biValuesByCands.resize(9);
    FOR_ALL(i) biValuesByCands[i].resize(i + 1);
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
        // // DEBUG
        // auto from = nodes[xn];
        // auto to = nodes[yn + cnt];
        // debugLog("(", from.x, ",", from.y, ",", from.target, ",", from.state,
        //          ") -> ");
        // debugLog("(", to.x, ",", to.y, ",", to.target, ",", to.state, ") \n
        // ");
        // // END DEBUG
        nodes[yn].edges.push_back(xn + cnt);

        // // DEBUG
        // from = nodes[yn];
        // to = nodes[xn + cnt];
        // debugLog("(", from.x, ",", from.y, ",", from.target, ",", from.state,
        //          ") -> ");
        // debugLog("(", to.x, ",", to.y, ",", to.target, ",", to.state, ") \n
        // ");
        // // END DEBUG
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
                // // DEBUG
                // auto from = nodes[cur];
                // auto to = nodes[other];
                // debugLog("(", from.x, ",", from.y, ",", from.target, ",",
                //          from.state, ") -> ");
                // debugLog("(", to.x, ",", to.y, ",", to.target, ",", to.state,
                //          ") \n ");
                // // END DEBUG
            }
            // different cell:
            FOR_ALL(oi) FOR_ALL(oj) {
                const Cell *other = &grid[oi][oj];
                if (!sees(cell, other)) continue;
                if (other->candidates[target]) {
                    int oth = lookup[std::make_pair(other, target)];
                    nodes[cur].edges.push_back(oth);
                    // // DEBUG
                    // auto from = nodes[cur];
                    // auto to = nodes[oth];
                    // debugLog("(", from.x, ",", from.y, ",", from.target, ",",
                    //          from.state, ") -> ");
                    // debugLog("(", to.x, ",", to.y, ",", to.target, ",",
                    //          to.state, ") \n ");
                    // // END DEBUG
                }
            }
        }
    }
}
Grid::Grid(std::string gridPattern) {
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

    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            grid[i][j].x = i;
            grid[i][j].y = j;
        }
    }
    strongLinks.resize(9);
    filled.resize(3);
    for (int houseType : {0, 1, 2}) {
        filled[houseType].resize(9);
    }
    
    updateCandCouldBe();
    updateBiValues();
    strongLinks.resize(9);
    updateStrongLinks();
    updateGraph();
    filled.resize(3);
    for(auto &house : filled){
        house.resize(9);
    }
    updateFilled();
}

Grid::Grid(int difficulty) {
    grid.resize(9);
    for (auto &row : grid) row.resize(9);

    // TODO: generate sudoku of given difficulty
    // NOTE: during nextStep() and execution, if a value is set, elinimate all
    // the same candidates see that.
}

std::string Grid::toString() {
    std::string res = "";
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            res += grid[i][j].value + '0';
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
    // TODO: call solvers in sequence from easy to hard;
    // TODO: init infos for solvers
    updateBiValues();
    updateStrongLinks();
    // once find a solution, return it;
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
    for(auto &houses : filled){
        for(auto &house : houses ){
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