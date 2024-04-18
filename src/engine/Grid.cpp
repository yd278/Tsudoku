#include "Grid.h"

#include <bitset>
#include <vector>
#include "util.h"
#include "solvers/DLX.h"

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
            if (grid[i][j].ans != res[i * 9 + j] - '0')
                throw std::invalid_argument(
                    "Invalid Sudoku: provided answer doesn't match");
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

std::string Grid::nextStep() {
    // TODO: call solvers in sequence from easy to hard;
    // once find a solution, return it;
    return "";
}


const Cell & Grid::getCell(int x, int y) const{
    return grid[x][y];
}
