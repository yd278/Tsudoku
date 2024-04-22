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
                    grid.addInst(0x20 + order - 2);
                    grid.setExec(false);
                    if (houseType) {
                        FOR_ALL(cover) {
                            if (filter[cover]) {
                                grid.addInst(cover << 4 | 0xF);
                            }
                        }
                        for (int i = 0; i < order; i++)
                            grid.addInst(0xf0 | base[i]);
                    } else {
                        for (int i = 0; i < order; i++)
                            grid.addInst(base[i] << 4 | 0xF);
                        FOR_ALL(cover) {
                            if (filter[cover]) {
                                grid.addInst(0xF0 | cover);
                            }
                        }
                    }
                    grid.addInst(target);
                    // eliminate candidates
                    bool flag = false;
                    FOR_ALL(cover) {
                        if (!filter[cover]) continue;
                        FOR_ALL(cell) {
                            bool inBase = false;
                            for (int i = 0; i < order; i++)
                                if (cell == base[i]) inBase = true;
                            if (inBase) continue;
                            auto tmp = grid.getCell(
                                convert(cover, cell, 1 - houseType));
                            if (tmp->candidates[target]) {
                                grid.addExec(encodePos(tmp) << 8 | target);
                                flag = true;
                            }
                        }
                    }
                    if (flag) {
                        grid.sortExec();
                        grid.addExecToInst();
                        return;
                    }
                }
            }
        }
    }
}