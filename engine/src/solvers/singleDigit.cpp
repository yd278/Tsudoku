#include "solvers/singleDigit.h"

#include <utility>
#include <vector>

#include "Grid.h"
#include "util.h"

void findTurbotFish(Grid &grid) {
    grid.initInsAndExe();
    auto allStrongLinks = grid.getStrongLinks();
    FOR_ALL(target) {
        // iterate two strong links
        auto &strongLinks = (*allStrongLinks)[target];
        int n = strongLinks.size();
        for (int i = 0; i < n; i++) {
            auto link1 = std::vector<const Cell *>{strongLinks[i].first,
                                                   strongLinks[i].second};

            for (int j = i + 1; j < n; j++) {
                auto link2 = std::vector<const Cell *>{strongLinks[j].first,
                                                       strongLinks[j].second};
                // check if either sees each other
                for (int b1 : {0, 1}) {
                    for (int b2 : {0, 1}) {
                        if (sees(link1[b1], link2[b2]) &&
                            link1[b1] != link2[b2]) {
                            std::vector<const Cell *> bridge = {link1[b1],
                                                                link2[b2]};
                            std::vector<const Cell *> pincer = {link1[1 - b1],
                                                                link2[1 - b2]};

                            // two pincers should not see each other
                            if (sees(pincer[0], pincer[1])) continue;
                            // turbot fish pattern find;

                            // determine order
                            if (cmp(bridge[1], bridge[0])) {
                                std::swap(bridge[0], bridge[1]);
                                std::swap(pincer[0], pincer[1]);
                            }

                            // determine type:
                            int type = 0;  // 0 SC, 1 Kite, 2 TF
                            if (link1[0]->x != link1[1]->x &&
                                link1[0]->y !=
                                    link1[1]
                                        ->y) {  // strong link in the same box
                                type = 2;
                            } else if (link2[0]->x != link2[1]->x &&
                                       link2[0]->y != link2[1]->y) {
                                type = 2;
                            } else {
                                int houseType1 =
                                    link1[0]->y ==
                                    link1[1]->y;  // 1 :column; 0 :row
                                int houseType2 = link2[0]->y == link2[1]->y;
                                type = houseType1 != houseType2;
                            }

                            // set insts;
                            grid.initInsAndExe();

                            FOR_ALL(ei) FOR_ALL(ej) {
                                if (sees(pincer[0], ei, ej) &&
                                    sees(pincer[1], ei, ej)) {
                                    if (grid.getCell(ei, ej)
                                            ->candidates[target]) {
                                        grid.addExec(encodePos(ei, ej), target);
                                    }
                                }
                            }
                            grid.setExec(false);
                            if (grid.emptyExec()) continue;
                            grid.addInst(0x50 + type);

                            for (int index : {0, 1})
                                grid.addInst(encodePos(bridge[index]));
                            for (int index : {0, 1})
                                grid.addInst(encodePos(pincer[index]));
                            grid.addInst(target);

                            grid.sortExec();
                            grid.addExecToInst();
                            return;
                        }
                    }
                }
            }
        }
    }
}
void findEmptyRectangle(Grid &grid) {
    FOR_ALL(target) {
        FOR_ALL(box) {
            if(grid.getFilled()->at(2)[box][target])continue;
            int cnt = 0;
            std::vector<const Cell *> cells;
            FOR_ALL(cellId) {
                auto cell = grid.getCell(2, box, cellId);
                if (cell->candidates[target]) cells.push_back(cell);
            }
            int row, col;
            bool found;
            int rOffset = (box / 3) * 3;
            int cOffset = (box % 3) * 3;
            for (int tRow = rOffset; tRow < rOffset + 3; tRow++) {
                found = false;
                for (int tCol = cOffset; tCol < cOffset + 3; tCol++) {
                    // check if:
                    //   1. no cells outside tRow and tCol
                    //   2. at least one more cell in the row and the column
                    bool rowFlag = false;
                    bool colFlag = false;
                    bool outside = false;
                    for (auto c : cells) {
                        if (c->y != tCol && c->x != tRow) {
                            outside = true;
                            break;
                        }
                        if (c->x == tRow && c->y == tCol)
                            continue;  // same cell, doesn't count
                        if (c->x == tRow) rowFlag = true;
                        if (c->y == tCol) colFlag = true;
                    }

                    if (outside) continue;  // check next tR and tC
                    if (rowFlag && colFlag) {
                        row = tRow;
                        col = tCol;
                        found = true;
                        break;
                    }
                }
                if (found) break;
            }
            if (!found) continue;  // empty rectangle not found, check next box
            // empty rect pattern found:

            // try row first;
            FOR_ALL(cellId) {
                auto pointed = grid.getCell(0, row, cellId);
                if (pointed->y / 3 == box % 3) continue;  // not in the same box
                if (pointed->SL[target + 9] != nullptr) {
                    auto other =
                        pointed->SL[target + 9];  // get the strong linked cell
                                                  // in the same column
                    int otherRow = other->x;
                    if (otherRow / 3 == box / 3)
                        continue;  // not in the same level
                    auto toEli = grid.getCell(otherRow, col);
                    if (toEli->candidates[target]) {
                        grid.initInsAndExe();
                        grid.setExec(false);
                        grid.addInst(0x53);
                        grid.addInst((row << 4) | 0xf);
                        grid.addInst(0xF0 | col);
                        grid.addInst(encodePos(pointed));
                        grid.addInst(encodePos(other));
                        grid.addInst(target);

                        grid.addExec(encodePos(toEli), target);
                        grid.addExecToInst();
                        return;
                    }
                }
            }

            // then try column
            FOR_ALL(cellId) {
                auto pointed = grid.getCell(1, col, cellId);
                if (pointed->x / 3 == box / 3) continue;  // not in the same box
                if (pointed->SL[target] != nullptr) {
                    auto other = pointed->SL[target];  // get the strong linked
                                                       // cell in the same row
                    int otherCol = other->y;
                    if (otherCol / 3 == box % 3)
                        continue;  // not in the same level
                    auto toEli = grid.getCell(row, otherCol);
                    if (toEli->candidates[target]) {
                        grid.initInsAndExe();
                        grid.setExec(false);
                        grid.addInst(0x53);
                        grid.addInst(0xF0 | col);
                        grid.addInst((row << 4) | 0xf);
                        grid.addInst(encodePos(pointed));
                        grid.addInst(encodePos(other));
                        grid.addInst(target);

                        grid.addExec(encodePos(toEli), target);
                        grid.addExecToInst();
                        return;
                    }
                }
            }
        }
    }
}