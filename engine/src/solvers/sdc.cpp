#include "solvers/sdc.h"

#include <algorithm>
#include <bitset>
#include <cstdint>
#include <tuple>
#include <utility>
#include <vector>

#include "Grid.h"
#include "util.h"
typedef std::vector<const Cell *> cellSet;

void findAllValid(
    Grid &grid, cellSet &all,
    std::vector<std::tuple<cellSet, cellSet, std::bitset<9>>> &valid,
    std::bitset<9> &CUnion, int CSize) {
    for (int size = 1; size < all.size(); size++) {
        std::vector<bool> mask(all.size());
        std::fill(mask.begin(), mask.begin() + size, true);
        do {
            cellSet tmpCR, tmpCRE;
            std::bitset<9> CRUnion;
            for (int i = 0; i < all.size(); i++) {
                if (mask[i]) {
                    tmpCR.push_back(all[i]);
                    CRUnion = CRUnion | all[i]->candidates;
                } else {
                    tmpCRE.push_back(all[i]);
                }
            }
            if ((CRUnion | CUnion).count() == CSize + size + 1) {
                auto tmpPair = std::make_tuple(
                    std::move(tmpCR), std::move(tmpCRE), std::move(CRUnion));
                valid.push_back(std::move(tmpPair));
            }

        } while (std::prev_permutation(mask.begin(), mask.end()));
    }
}

void findSDC(Grid &grid) {
    FOR_ALL(box) {
        for (int lineType : {0, 1}) {
            int lineOffset = (box / 3) * 3;
            int cellOffset = (box % 3) * 3;
            if (lineType) std::swap(lineOffset, cellOffset);
            for (int lineNum : {0, 1, 2}) {
                cellSet intersect;
                for (int cellNum : {0, 1, 2}) {
                    auto cell = grid.getCell(lineType, lineOffset + lineNum,
                                             cellOffset + cellNum);
                    if (!cell->value) intersect.push_back(cell);
                }
                if (intersect.size() < 2) continue;
                std::vector<cellSet> choosenIntersect;
                if (intersect.size() == 2)
                    choosenIntersect.push_back(std::move(intersect));
                else {
                    choosenIntersect.push_back(
                        cellSet{intersect[0], intersect[1]});
                    choosenIntersect.push_back(
                        cellSet{intersect[0], intersect[2]});
                    choosenIntersect.push_back(
                        cellSet{intersect[1], intersect[2]});
                    choosenIntersect.push_back(std::move(intersect));
                }
                for (auto C : choosenIntersect) {
                    // check if AALS condition satisfied
                    std::bitset<9> CUnion;
                    for (auto cell : C) {
                        CUnion = CUnion | cell->candidates;
                    }
                    if (CUnion.count() < C.size() + 2) continue;

                    // find all R-cells
                    cellSet allR;
                    FOR_ALL(cellNum) {
                        auto cell = grid.getCell(lineType, lineNum + lineOffset,
                                                 cellNum);
                        if (findBox(cell) == box) continue;
                        if (cell->value) continue;
                        allR.push_back(cell);
                    }

                    // select and check CR: CR and C form a ALS

                    std::vector<std::tuple<cellSet, cellSet, std::bitset<9>>>
                        validCR;
                    findAllValid(grid, allR, validCR, CUnion, C.size());
                    if (validCR.empty()) continue;
                    // find all B-cells;
                    cellSet allB;
                    FOR_ALL(cellNum) {
                        auto cell = grid.getCell(2, box, cellNum);
                        int cmp = lineType ? cell->y : cell->x;
                        if (cmp == lineNum + lineOffset) continue;
                        if (cell->value) continue;
                        allB.push_back(cell);
                    }

                    // select and check CB: CB and C form a ALS

                    std::vector<std::tuple<cellSet, cellSet, std::bitset<9>>>
                        validCB;
                    findAllValid(grid, allB, validCB, CUnion, C.size());
                    if (validCB.empty()) continue;

                    // cross-validation (?)
                    for (auto &CR : validCR) {
                        auto &CRUnion = std::get<2>(CR);
                        for (auto &CB : validCB) {
                            auto &CBUnion = std::get<2>(CB);
                            if((CRUnion&CBUnion&CUnion).count()!=0) continue; // not a valid su de coq
                            // valid su de coq found

                            std::vector<uint16_t> exes;

                            // we can eliminate candidates in CR or (in C but not in CB)
                            auto REmask = CUnion & (~CBUnion);
                            auto RExes = std::get<1>(CR);
                            for(auto exe : RExes){
                                auto exeCands = exe->candidates & REmask;
                                FOR_ALL(cand){
                                    if(exeCands[cand]) exes.push_back(encodeExe(exe, cand));
                                }
                            }

                            // and eliminate candidates in CB, or (in C/CR)
                            auto BEmask = CUnion &(~CRUnion);
                            auto BExes = std::get<1>(CB);
                            for(auto exe : BExes){
                                auto exeCands = exe->candidates & BEmask;
                                FOR_ALL(cand){
                                    if(exeCands[cand]) exes.push_back(encodeExe(exe, cand));
                                }
                            }

                            if(!exes.empty()){
                                // useful sdq found
                                grid.addInst(0x92);
                                grid.addInst(C.size());
                                for(auto c : C) grid.addInst(encodePos(c));
                                auto CRPos = std::get<0>(CR);
                                grid.addInst(CRPos.size());
                                for(auto c : CRPos) grid.addInst(encodePos(c));
                                
                                auto CBPos = std::get<0>(CB);
                                grid.addInst(CBPos.size());
                                for(auto c : CBPos) grid.addInst(encodePos(c));
                                for(auto exe : exes) grid.addExec(exe);
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
}