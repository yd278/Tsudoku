#include "solvers/uniqueness.h"

#include <bitset>
#include <functional>
#include <utility>
#include <vector>

#include "util.h"
#include "util_const.h"

void uniquenessTestType1(Grid &grid) {
    // no more than two bivalues with same candidates in one house:
    // as we always execute naked pair before uniqueness test, these two
    // bivalues are strong linked

    auto bivaluesMap = grid.getBiValuesByCands();
    FOR_ALL(x) for (int y = 0; y < x; y++) {
        auto bivalues = (*bivaluesMap)[x][y];
        for (auto cell : bivalues) {
            if (!cell->SL[x]) continue;
            if (!cell->SL[y]) continue;
            if (cell->SL[x] != cell->SL[y]) continue;

            if (!cell->SL[x + 9]) continue;
            if (!cell->SL[y + 9]) continue;
            if (cell->SL[x + 9] != cell->SL[y + 9]) continue;
            // type1 pattern found:
            auto pincer1 = cell->SL[x];
            auto pincer2 = cell->SL[x + 9];
            if ((cell->y / 3 != pincer1->y / 3) &&
                (cell->x / 3 != pincer2->x / 3))
                continue;
            auto exec = grid.getCell(pincer2->x, pincer1->y);
            if (exec->value) continue;
            if (exec->candCouldBe[x] && exec->candCouldBe[y]) {
                grid.initInsAndExe();
                grid.addInst(0x60);
                int row1 = cell->x;
                int row2 = pincer2->x;
                if (row1 > row2) std::swap(row1, row2);
                grid.addInst((row1 << 4) | 0xF);
                grid.addInst((row2 << 4) | 0xF);

                int col1 = cell->y;
                int col2 = pincer1->y;
                if (col1 > col2) std::swap(col1, col2);
                grid.addInst(0xF0 | col1);
                grid.addInst(0xf0 | col2);

                grid.addInst(y);
                grid.addInst(x);
                bool flag = false;
                if (exec->candidates[y])
                    grid.addExec(encodePos(exec), y), flag = true;
                if (exec->candidates[x])
                    grid.addExec(encodePos(exec), x), flag = true;

                if (flag) {
                    grid.addExecToInst();
                    return;
                }
            }
        }
    }
}

void findPossibleURViaNakedPair(
    Grid &grid, std::function<bool(Grid &, int, int, int, int, int, int, int,
                                   bool, const Cell *, const Cell *)>
                    processLogic) {
    auto bivaluesMap = grid.getBiValuesByCands();
    FOR_ALL(x) for (int y = 0; y < x; y++) {
        auto bivalues = (*bivaluesMap)[x][y];
        for (auto cell : bivalues) {
            for (int houseType : {0, 1}) {
                if (cell->SL[x + houseType * 9] == nullptr) continue;
                if (cell->SL[y + houseType * 9] == nullptr) continue;
                if (cell->SL[x + houseType * 9] != cell->SL[y + houseType * 9])
                    continue;
                // naked pair found
                // VC coordinate perpendicular to the naked pair
                int VC1 = houseType ? cell->x : cell->y;
                int VC2 = houseType ? cell->SL[x + 9]->x : cell->SL[x]->y;
                bool URCondition = (VC1 / 3) == (VC2 / 3);
                // HC coordinate parallel to the naked pair

                auto HC0 = houseType ? cell->y : cell->x;
                FOR_ALL(HC) {
                    if (!URCondition && (HC0 / 3) != (HC / 3)) continue;
                    if (HC == HC0) continue;
                    const Cell *tail1 = grid.getCell(houseType, HC, VC1);  //
                    if (tail1->value) continue;
                    if (!tail1->candCouldBe[x]) continue;
                    if (!tail1->candCouldBe[y]) continue;
                    const Cell *tail2 = grid.getCell(houseType, HC, VC2);
                    if (tail2->value) continue;
                    if (!tail2->candCouldBe[x]) continue;
                    if (!tail2->candCouldBe[y]) continue;
                    // UR found
                    if (processLogic(grid, houseType, HC0, HC, VC1, VC2, x, y,
                                     URCondition, tail1, tail2))
                        return;
                }
            }
        }
    }
}

bool checkURType2(Grid &grid, int houseType, int HC0, int HC, int VC1, int VC2,
                  int x, int y, bool URCondition, const Cell *tail1,
                  const Cell *tail2) {
    // UR found
    // check if satisfies UT type2
    int extra = -1;
    bool moreThanOneExtra = false;
    FOR_ALL(cand) {
        if (tail1->candidates[cand]) {
            if (cand == x || cand == y) continue;
            if (extra == -1)
                extra = cand;
            else {
                moreThanOneExtra = true;
                break;
            }
        }
    }
    if (moreThanOneExtra) return false;

    FOR_ALL(cand) {
        if (tail2->candidates[cand]) {
            if (cand == x || cand == y) continue;
            if (cand != extra) {
                moreThanOneExtra = true;
                break;
            }
        }
    }
    if (moreThanOneExtra) return false;
    // UT type2 found
    if (VC1 > VC2) std::swap(VC1, VC2);
    grid.initInsAndExe();
    grid.setExec(false);
    grid.addInst(0x61);
    grid.addInst(encodeLine(houseType, HC0));
    grid.addInst(encodeLine(houseType, HC));
    grid.addInst(encodeLine(1 - houseType, VC1));
    grid.addInst(encodeLine(1 - houseType, VC2));
    grid.addInst(y);
    grid.addInst(x);
    grid.addInst((URCondition ? 0xF0 : 0) | extra);
    bool flag = false;
    FOR_ALL(ei) FOR_ALL(ej) {
        if (!sees(tail1, ei, ej)) continue;
        ;
        if (!sees(tail2, ei, ej)) continue;
        ;
        auto exec = grid.getCell(ei, ej);
        if (exec == tail1 || exec == tail2) continue;
        ;
        if (exec->candidates[extra])
            grid.addExec(encodePos(exec), extra), flag = true;
    }
    if (flag) {
        grid.addExecToInst();
        return true;
    }
    return false;
}

bool findNakedSubsetByPerm(std::bitset<9> &virtualCell,
                           std::vector<const Cell *> &virtualLine, Grid &grid,
                           int houseType, int HC0, int HC, int VC1, int VC2,
                           int x, int y) {
    int lowerBound = virtualCell.count();
    std::vector<const Cell *> hints;  // naked subset (except virtual)
    hints.reserve(9);
    for (int size = lowerBound; size < virtualLine.size() + 1; size++) {
        std::vector<bool> v(virtualLine.size());
        std::fill(v.begin(), v.begin() + size - 1, true);
        do {
            auto unionSet = virtualCell;
            for (int i = 0; i < virtualLine.size(); i++) {
                if (v[i]) unionSet = unionSet | virtualLine[i]->candidates;
            }
            if (unionSet.count() == size) {
                // naked subset found
                //  test executees
                grid.initInsAndExe();
                for (int i = 0; i < virtualLine.size(); i++) {
                    if (v[i]) continue;
                    auto tmp = virtualLine[i]->candidates & unionSet;
                    if (tmp.count() != 0) {
                        FOR_ALL(cand) {
                            if (!tmp[cand]) continue;
                            grid.addExec(encodePos(virtualLine[i]), cand);
                        }
                    }
                }
                if (!grid.emptyExec()) {
                    grid.addInst(0x62);
                    grid.addInst(encodeLine(houseType, HC0));
                    grid.addInst(encodeLine(houseType, HC));
                    grid.addInst(encodeLine(1 - houseType, VC1));
                    grid.addInst(encodeLine(1 - houseType, VC2));
                    grid.addInst(y);
                    grid.addInst(x);
                    grid.addInst(size);
                    for (int i = 0; i < virtualLine.size(); i++) {
                        if (v[i]) grid.addInst(encodePos(virtualLine[i]));
                    }
                    grid.addExecToInst();
                    return true;
                }
            }

        } while (std::prev_permutation(v.begin(), v.end()));
    }
    return false;
}

bool checkURType3(Grid &grid, int houseType, int HC0, int HC, int VC1, int VC2,
                  int x, int y, bool URCondition, const Cell *tail1,
                  const Cell *tail2) {
    // now we DO get everything we need for instructions - the information about
    // the UR we just need to check if this UR can be used to eliminate some
    // candidates using type 3 logic
    //  if we can get some executees, we can write and return
    //  if we cannot , return false;
    // get virtual line

    std::vector<const Cell *> virtualLine;
    FOR_ALL(index) {
        auto cell = grid.getCell(houseType, HC, index);
        if (cell == tail1 || cell == tail2) continue;
        if (cell->value) continue;
        virtualLine.push_back(cell);
    }
    std::bitset<9> virtualCell;
    FOR_ALL(cand) {
        if (cand == x || cand == y) continue;
        if (tail1->candidates[cand] || tail2->candidates[cand])
            virtualCell[cand] = true;
    }

    if (findNakedSubsetByPerm(virtualCell, virtualLine, grid, houseType, HC0,
                              HC, VC1, VC2, x, y))
        return true;
    int box = findBox(tail1);
    virtualLine.clear();
    FOR_ALL(index) {
        auto cell = grid.getCell(2, box, index);
        if (cell == tail1 || cell == tail2) continue;
        if (cell->value) continue;
        virtualLine.push_back(cell);
    }
    if (findNakedSubsetByPerm(virtualCell, virtualLine, grid, houseType, HC0,
                              HC, VC1, VC2, x, y))
        return true;

    return false;
}

bool checkURType4(Grid &grid, int houseType, int HC0, int HC, int VC1, int VC2,
                  int x, int y, bool URCondition, const Cell *tail1,
                  const Cell *tail2) {
    if (!URCondition) return false;
    int box = findBox(tail1);
    std::bitset<9> mask;
    FOR_ALL(index) {
        auto cell = grid.getCell(houseType, HC, index);
        if (cell->value) continue;
        if (cell == tail1 || cell == tail2) continue;
        mask = mask | cell->candidates;
    }
    FOR_ALL(index) {
        auto cell = grid.getCell(2, box, index);
        if (cell->value) continue;
        if (cell == tail1 || cell == tail2) continue;
        mask = mask | cell->candidates;
    }

    grid.initInsAndExe();
    grid.setExec(false);
    grid.addInst(0x63);
    grid.addInst(encodeLine(houseType, HC0));
    grid.addInst(encodeLine(houseType, HC));
    grid.addInst(encodeLine(1 - houseType, VC1));
    grid.addInst(encodeLine(1 - houseType, VC2));
    for (int tmp : {x, y}) {
        int other = tmp==x?y:x;
        if (!mask[tmp]) {
            grid.addInst(tmp);
            grid.addInst(other);

            if (tail1->candidates[other]) grid.addExec(tail1, other);
            if (tail2->candidates[other]) grid.addExec(tail2, other);
            if (!grid.emptyExec()) {
                grid.sortExec();
                grid.addExecToInst();
                return true;
            }
        }
    }

    return false;
}
void uniquenessTestType2(Grid &grid) {
    findPossibleURViaNakedPair(grid, checkURType2);
}
void uniquenessTestType3(Grid &grid) {
    findPossibleURViaNakedPair(grid, checkURType3);
}

void uniquenessTestType4(Grid &grid) {
    findPossibleURViaNakedPair(grid, checkURType4);
}
void uniquenessTestType5(Grid &grid) {
    
}
void findHiddenRectangle(Grid &grid) {}
void avoidableRectangle1(Grid &grid) {}
void avoidableRectangle2(Grid &grid) {}

void bivalueUniversalGraveP1(Grid &grid) {
    // check  all bi-values
    const Cell *tri = nullptr;
    FOR_ALL(i) FOR_ALL(j) {
        auto cell = grid.getCell(i, j);
        if (cell->value) continue;
        if (cell->candidates.count() == 3) {
            if (tri == nullptr)
                tri = cell;
            else
                return;

        } else if (cell->candidates.count() != 2)
            return;
    }
    if (tri == nullptr) return;

    // check extra candidate
    int extra = -1;

    FOR_ALL(can) {
        if (!tri->candidates[can]) continue;
        // test row:
        int cnt = 0;
        FOR_ALL(col) {
            if (grid.getCell(tri->x, col)->candidates[can]) cnt++;
        }
        if (cnt == 2) continue;  // either not BUG, or not extra
        if (cnt == 3) {
            if (extra == -1)
                extra = can;
            else
                return;  // not BUG
        }
        if (cnt != 3) return;  // not BUG
        cnt = 0;
        FOR_ALL(row) {
            if (grid.getCell(row, tri->y)->candidates[can]) cnt++;
        }
        if (cnt != 3) return;

        int box = findBox(tri);
        cnt = 0;
        FOR_ALL(cell) {
            if (grid.getCell(2, box, cell)->candidates[can]) cnt++;
        }
        if (cnt != 3) return;
    }
    if (extra == -1) return;

    // check bug

    for (int houseType : {0, 1, 2}) {
        FOR_ALL(houseId) {
            std::vector<int> count(9);
            FOR_ALL(cellId) {
                auto cell = grid.getCell(houseType, houseId, cellId);

                if (cell->value) continue;
                FOR_ALL(can) {
                    if (cell->candidates[can]) count[can]++;
                }
                if (cell == tri) count[extra]--;
            }
            FOR_ALL(can) {
                if (count[can] != 0 && count[can] != 2) return;
            }
        }
    }

    // bug find
    grid.initInsAndExe();
    grid.setExec(true);
    grid.addInst(0x69);
    grid.addExec(encodePos(tri), extra);
    grid.addExecToInst();
}