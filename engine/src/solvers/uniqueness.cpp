#include "solvers/uniqueness.h"

#include "util.h"

// TODO: unique rectangle need to be in same tower or same level, fix this
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
            auto exec = grid.getCell(pincer2->x, pincer1->y);
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
                if (exec->candidates[y]) grid.addExec(encodePos(exec), y);
                if (exec->candidates[x]) {
                    grid.addExec(encodePos(exec), x);
                }

                grid.addExecToInst();
                return;
            }
        }
    }
}

void uniquenessTestType2(Grid &grid) {
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
                    if (moreThanOneExtra) continue;

                    FOR_ALL(cand) {
                        if (tail2->candidates[cand]) {
                            if (cand == x || cand == y) continue;
                            if (cand != extra) {
                                moreThanOneExtra = true;
                                break;
                            }
                        }
                    }
                    if (moreThanOneExtra) continue;
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
                        if (!sees(tail2, ei, ej)) continue;
                        auto exec = grid.getCell(ei, ej);
                        if (exec == tail1 || exec == tail2) continue;
                        if (exec->candidates[extra])
                            grid.addExec(encodePos(exec), extra), flag = true;
                    }
                    if (flag) {
                        grid.addExecToInst();
                        return;
                    }
                }
            }
        }
    }
}
void uniquenessTestType3(Grid &grid) {}
void uniquenessTestType4(Grid &grid) {}
void uniquenessTestType5(Grid &grid) {}
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