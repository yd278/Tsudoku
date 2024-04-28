#include "solvers/fish.h"

#include "util.h"
#include "util_const.h"

void findFish(Grid &grid, int order) {
    std::vector<const std::vector<std::vector<int>> *> combos = {
        &ALL_PAIRS, &ALL_TRIPLES, &ALL_QUADRUPLETS};
    FOR_ALL(target) {
        for (int houseType : {0, 1}) {
            for (auto &base : *combos[order - 2]) {
                std::bitset<9> filter;
                bool used = false;
                FOR_ALL(cell) {
                    for (int i = 0; i < order; i++) {
                        auto tmpC =
                            grid.getCell(convert(base[i], cell, houseType));
                        if (tmpC->value == target + 1) {
                            used = true;
                            break;
                        }
                        if (tmpC->candidates[target]) filter.set(cell);
                    }
                }
                if (used) continue;
                if (filter.count() == order) {
                    // fish found
                    grid.initInsAndExe();
                    grid.setExec(false);

                    FOR_ALL(cover) {
                        if (!filter[cover]) continue;
                        FOR_ALL(cell) {
                            bool inBase = false;
                            for (int i = 0; i < order; i++)
                                if (cell == base[i]) inBase = true;
                            if (inBase) continue;
                            auto tmp = grid.getCell(1 - houseType, cover, cell);
                            if (tmp->candidates[target])
                                grid.addExec(encodePos(tmp), target);
                        }
                    }
                    if (!grid.emptyExec()) {
                        grid.addInst(0x20 + order - 2);

                        for (auto b : base) {
                            grid.addInst(encodeLine(houseType, b));
                        }
                        FOR_ALL(cover) {
                            if (!filter[cover]) continue;
                            grid.addInst(encodeLine(1 - houseType, cover));
                        }
                        grid.addInst(target);
                        grid.sortExec();
                        grid.addExecToInst();
                        return;
                    }

                    // eliminate candidates
                }
            }
        }
    }
}

void findFinedFish(Grid &grid, int order) {
    auto filled = grid.getFilled();
    std::vector<const std::vector<std::vector<int>> *> combos = {
        &ALL_PAIRS, &ALL_TRIPLES, &ALL_QUADRUPLETS};

    FOR_ALL(target) {
        for (int houseType : {0, 1}) {
            for (auto &base : *combos[order - 2]) {
                bool invalidBase = false;
                for (int index : base) {
                    if (filled->at(houseType)[index][target]) {
                        invalidBase = true;
                        break;
                    }
                }
                if (invalidBase) continue;

                for (auto &cover : *combos[order - 2]) {
                    bool invalidCover = false;
                    for (int index : cover) {
                        if (filled->at(1 - houseType)[index][target]) {
                            invalidCover = true;
                            break;
                        }
                    }
                    if (invalidCover) continue;
                    //  base set and cover set fixed, check if a finned fish:
                    //  pick fins and check if fins has something in their
                    //  intersect;
                    
                    bool noIntersect = false;
                    std::vector<const Cell *> fins;
                    std::vector<std::vector<bool>> finIntersect(9);
                    for (auto &row : finIntersect) row.resize(9);

                    FOR_ALL(i) FOR_ALL(j) finIntersect[i][j] = true;
                    for (auto baseIndex : base) {
                        int ptr = 0;
                        FOR_ALL(cellId) {
                            if (ptr < order && cellId == cover[ptr]) {
                                ptr++;
                                continue;
                            }
                            auto cell =
                                grid.getCell(houseType, baseIndex, cellId);
                            if (!cell->candidates[target]) continue;
                            

                            // a fin is found
                            fins.push_back(cell);
                            int cnt = 0;
                            FOR_ALL(i) FOR_ALL(j) {
                                if (!sees(cell, i, j))
                                    finIntersect[i][j] = false;
                                if (finIntersect[i][j]) cnt++;
                            }
                            if (cnt == 0) {
                                noIntersect = true;
                                break;
                            }
                        }
                        if (noIntersect) break;
                    }
                    if (noIntersect) continue;

                    // check if fin and cover have intersect and contains target
                    // as candidate
                    std::vector<const Cell *> exes;
                    for (int coverIndex : cover) {
                        int ptr = 0;
                        FOR_ALL(cellId) {
                            if (ptr < order && cellId == base[ptr]) {
                                ptr++;
                                continue;
                            }
                            auto pos =
                                convert(coverIndex, cellId, 1 - houseType);
                            if (finIntersect[pos.first][pos.second]) {
                                if (grid.getCell(pos)->candidates[target]) {
                                    exes.push_back(grid.getCell(pos));
                                }
                            }
                        }
                    }
                    if (exes.empty()) continue;

                    //  finally we have some executees:
                    grid.initInsAndExe();
                    grid.addInst(0x80 + order - 2);
                    for (auto baseIndex : base) {
                        grid.addInst(encodeLine(houseType, baseIndex));
                    }
                    for (auto coverIndex : cover) {
                        grid.addInst(encodeLine(1 - houseType, coverIndex));
                    }
                    grid.addInst(fins.size());
                    for (auto fin : fins) {
                        grid.addInst(encodePos(fin));
                    }
                    grid.addInst(target);
                    for (auto exe : exes) {
                        grid.addExec(exe, target);
                    }
                    grid.addExecToInst();
                    return;
                }
            }
        }
    }
}