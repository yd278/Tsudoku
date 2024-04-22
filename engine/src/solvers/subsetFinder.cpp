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
                    grid.addInst(0x30 + cnt - 2);
                    for (int i = 0; i < cnt; i++) {
                        auto pos = convert(house, cells[i], houseType);
                        grid.addInst(encodePos(pos));
                    }
                    std::vector<int> subsetCandidates;
                    FOR_ALL(cand) {
                        if (candidateUnion[cand]) {
                            grid.addInst(cand);
                            subsetCandidates.push_back(cand);
                        }
                    }
                    // eliminations:
                    bool flag = false;
                    // iterate every cells in this house
                    FOR_ALL(cellId) {
                        // check if it's in the subset selected
                        bool inSubset = false;
                        for (int i = 0; i < cnt; i++) {
                            if (cellId == cells[i]) {
                                inSubset = true;
                                break;
                            }
                        }
                        if (inSubset) continue;
                        // it's not, take the cell;
                        auto pos = convert(house, cellId, houseType);
                        auto cell = grid.getCell(pos);
                        // it shouldn't be filled;
                        if (cell->value != 0) continue;
                        // determine what s
                        for (auto sc : subsetCandidates) {
                            if (cell->candidates[sc]) {
                                grid.addExec(encodePos(cell), sc);
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
                    grid.setExec(false);
                    grid.addInst(0x34 + cnt - 2);
                    std::vector<int> positionList;
                    // put pos into inst
                    FOR_ALL(i) {
                        if (positionUnion[i]) {
                            grid.addInst(
                                encodePos(convert(houseID, i, houseType)));
                            positionList.push_back(i);
                        }
                    }
                    // put cand into inst
                    for (auto cand : cands) {
                        grid.addInst(cand);
                    }

                    // eliminations:
                    bool flag = false;
                    for (auto pos : positionList) {
                        auto cell = grid.getCell(houseType, houseID, pos);
                        FOR_ALL(c) {
                            if (cell->candidates[c] && !candSet[c]) {
                                grid.addExec(encodePos(cell), c);
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
