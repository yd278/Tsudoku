#include "Grid.h"

#include <bitset>
#include <vector>

#include "solvers/DLX.h"
#include "util.h"

void Grid::updateStrongLinks() {
    strongLinks.clear();
    strongLinks.resize(9);
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
                    Cell& a = grid[tmp[0]->x][tmp[0]->y];
                    Cell& b = grid[tmp[1]->x][tmp[1]->y];
                    a.SL[houseType*9+target] = &b;
                    b.SL[houseType*9+target] = &a;

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

const Cell* Grid::getCell(int houseType,int houseID, int cellID) const{
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
    FOR_ALL(i) biValuesByCands[i].resize(i+1);
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (grid[i][j].candidates.count() == 2) {
                biValues.push_back(&grid[i][j]);
                std::vector<int> cands;
                FOR_ALL(can) if(grid[i][j].candidates[can])cands.push_back(can);
                biValuesByCands[cands[1]][cands[0]].push_back(&grid[i][j]);
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

    updateBiValues();
    updateStrongLinks();
}

Grid::Grid(int difficulty) {
    grid.resize(9);
    for (auto &row : grid) row.resize(9);

    // TODO: generate sudoku of given difficulty
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
