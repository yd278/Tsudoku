#include "solvers/subsetFinder.h"

#include "util.h"
#include "util_const.h"

void findNakedSubset(Grid &grid, int cnt) {
    std::vector<const std::vector<std::vector<int>> *> combos = {
        &ALL_PAIRS, &ALL_TRIPLES, &ALL_QUADRUPLETS};
    for (int houseType : {0, 1, 2}) {
        FOR_ALL(house) {
            for (auto cells : *combos[cnt - 2]) {
                bool hasValue = false;
                std::bitset<9> candidateUnion;
                for (int i = 0; i < cnt; i++) {
                    auto cell = grid.getCell(houseType, house, cells[i]);
                    if (cell->value != 0) {
                        hasValue = true;
                        break;
                    }
                    candidateUnion = candidateUnion | cell->candidates;
                }

                if (hasValue) continue;
                if (candidateUnion.count() == cnt) {
                    // naked subset found:
                    grid.initInsAndExe();
                    grid.setExec(false);
                    int ptr = 0;
                    FOR_ALL(cellId) {
                        if (ptr < cnt && cells[ptr] == cellId) {
                            ptr++;
                            continue;
                        }
                        auto cell = grid.getCell(houseType, house, cellId);
                        if (cell->value) continue;
                        FOR_ALL(target) {
                            if (!candidateUnion[target]) continue;
                            if (cell->candidates[target]) {
                                grid.addExec(cell, target);
                            }
                        }
                    }
                    if (!grid.emptyExec()) {
                        grid.addInst(0x30 + cnt - 2);
                        for (int i = 0; i < cnt; i++) {
                            auto pos = convert(house, cells[i], houseType);
                            grid.addInst(encodePos(pos));
                        }
                        FOR_ALL(cand) {
                            if (candidateUnion[cand]) {
                                grid.addInst(cand);
                            }
                        }
                        grid.sortExec();
                        grid.addExecToInst();
                        return;
                    }
                }
            }
        }
    }
}

void findHiddenSubset(Grid &grid, int cnt) {
    std::vector<std::bitset<9>> positions;
    std::vector<const std::vector<std::vector<int>> *> combos = {
        &ALL_PAIRS, &ALL_TRIPLES, &ALL_QUADRUPLETS};
    for (int houseType : {0, 1, 2}) {
        FOR_ALL(houseID) {
            //  get positions set;
            positions.clear();
            positions.resize(9);
            FOR_ALL(cellID) {
                auto cell = grid.getCell(houseType, houseID, cellID);
                if (cell->value != 0) continue;
                FOR_ALL(v) {
                    if (cell->candidates[v]) positions[v].set(cellID);
                }
            }

            for (auto cands : *combos[cnt - 2]) {
                std::bitset<9> positionUnion;
                std::bitset<9> candSet;

                // get position Union set, skip combos with used numbers
                bool used = false;
                for (auto cand : cands) {
                    if (positions[cand].count() == 0) {
                        used = true;
                        break;
                    }
                    positionUnion = positionUnion | positions[cand];
                    candSet.set(cand);
                }
                if (used) continue;

                if (positionUnion.count() == cnt) {
                    // hidden Subset found
                    grid.initInsAndExe();
                    FOR_ALL(pos) {
                        if (!positionUnion[pos]) continue;
                        auto cell = grid.getCell(houseType, houseID, pos);
                        FOR_ALL(c) {
                            if (cell->candidates[c] && !candSet[c]) {
                                grid.addExec(encodePos(cell), c);
                            }
                        }
                    }
                    if (grid.emptyExec()) continue;
                    grid.setExec(false);
                    grid.addInst(0x34 + cnt - 2);
                    // put pos into inst
                    FOR_ALL(i) {
                        if (positionUnion[i]) {
                            grid.addInst(
                                encodePos(convert(houseID, i, houseType)));
                        }
                    }
                    // put cand into inst
                    for (auto cand : cands) {
                        grid.addInst(cand);
                    }

                    // eliminations:
                    grid.sortExec();
                    grid.addExecToInst();
                    return;
                }
            }
        }
    }
}
