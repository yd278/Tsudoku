#include "solvers/wings.h"
#include "util.h"
inline bool checkBiValue(const Cell* cell, int x,
                         int y) {
    return cell->candidates[x] && cell->candidates[y];
}

void findXYWing(Grid &grid) {
    grid.initInsAndExe();
    FOR_ALL(x) {
        for (int y = x + 1; y < 9; y++) {
            FOR_ALL(z) {
                if (z == x || z == y) continue;
                // for fixed xyz
                for (auto pivot : *grid.getBiValues()) {
                    if (!checkBiValue(pivot, x, y)) continue;
                    // find the pivot with xy
                    for (auto xPincers : *grid.getBiValues()) {
                        if (!checkBiValue(xPincers, x, z)) continue;
                        if (!sees(pivot, xPincers)) continue;
                        // find the xPincers with xz
                        for (auto yPincers : *grid.getBiValues()) {
                            if (!checkBiValue(yPincers, y, z)) continue;
                            if (!sees(pivot, yPincers)) continue;
                            // find the yPincers with yz
                            if (sees(xPincers, yPincers)) continue;
                            // find one
                            grid.initInsAndExe();
                            grid.setExec(false);
                            // setup instruction
                            grid.addInst(0x40);
                            grid.addInst(encodePos(pivot));
                            grid.addInst(encodePos(xPincers));
                            grid.addInst(encodePos(yPincers));
                            grid.addInst(x);
                            grid.addInst(y);
                            grid.addInst(z);
                            bool flag = false;
                            // setup elimination
                            FOR_ALL(i) FOR_ALL(j) {
                                if (sees(xPincers, i, j) &&
                                    sees(yPincers, i, j))
                                    if (grid.getCell(i, j)->candidates[z]) {
                                        auto bPos = encodePos(i, j);
                                        grid.addInst(bPos);
                                        grid.addInst(z);
                                        grid.addExec((bPos << 8) | z);
                                        flag = true;
                                    }
                            }

                            if (flag) return;
                        }
                    }
                }
            }
        }
    }
}

void findXYZWing(Grid &grid) {
    grid.initInsAndExe();
    FOR_ALL(x)
    for (int y = x + 1; y < 9; y++) {
        FOR_ALL(z) {
            if (z == x || z == y) continue;
            // for fixed xyz

            for (auto xPincers : *grid.getBiValues()) {
                if (!checkBiValue(xPincers, x, z)) continue;
                // find a xPincer
                for (auto yPincers : *grid.getBiValues()) {
                    if (!checkBiValue(yPincers, y, z)) continue;
                    if (sees(xPincers, yPincers)) continue;
                    // find a yPincer
                    FOR_ALL(i) FOR_ALL(j) {
                        if (!sees(xPincers, i, j) || !sees(yPincers, i, j))
                            continue;
                        auto pivot = grid.getCell(i, j);
                        if (pivot->candidates.count() != 3) continue;
                        if (!pivot->candidates[x]) continue;
                        if (!pivot->candidates[y]) continue;
                        if (!pivot->candidates[z]) continue;
                        // find a pivot

                        // set Instructions
                        grid.initInsAndExe();
                        grid.setExec(false);

                        // setup instruction
                        grid.addInst(0x41);
                        grid.addInst(encodePos(pivot));
                        grid.addInst(encodePos(xPincers));
                        grid.addInst(encodePos(yPincers));
                        grid.addInst(x);
                        grid.addInst(y);
                        grid.addInst(z);
                        // set execution
                        bool flag = false;
                        FOR_ALL(ti) FOR_ALL(tj) {
                            if (ti == i && tj == j)
                                continue;  // not the pivot itself

                            if (!sees(pivot, ti, tj)) continue;
                            if (!sees(xPincers, ti, tj)) continue;
                            if (!sees(yPincers, ti, tj)) continue;
                            if (grid.getCell(ti, tj)->candidates[z]) {
                                auto bPos = encodePos(ti, tj);
                                grid.addInst(bPos);
                                grid.addInst(z);
                                grid.addExec((bPos << 8) | z);
                                flag = true;
                            }
                        }
                        if (flag) return;
                    }
                }
            }
        }
    }
}


void findWWing(Grid &grid) {

    FOR_ALL(bridge) {
        for (auto link : grid.getStrongLinks()->at(bridge)) {
            
            for (auto &pincer1 : *grid.getBiValues()) {
                if (!pincer1->candidates[bridge]) continue;
                if (!sees(pincer1, link.first)) continue;
                
                int loose;
                FOR_ALL(t)
                if (pincer1->candidates[t] && t != bridge) {
                    loose = t;
                    break;
                }
                

                for (auto &pincer2 :*grid.getBiValues()) {
                    if (pincer2==pincer1) continue;
                    if (!checkBiValue(pincer2, bridge, loose)) continue;
                    if (!sees(pincer2, link.second)) continue;
                    if (sees(pincer1, pincer2)) continue;

                    
                    //  w-wing pattern found
                    grid.initInsAndExe();
                    grid.setExec(false);
                    // setup instruction
                    grid.addInst(0x42);
                    grid.addInst(encodePos(pincer1));
                    grid.addInst(encodePos(pincer2));
                    grid.addInst(bridge);
                    grid.addInst(loose);
                    grid.addInst(encodePos(link.first));
                    grid.addInst(encodePos(link.second));

                    // set executee
                    bool flag = false;
                    FOR_ALL(i) FOR_ALL(j) {
                        if (!sees(pincer1, i, j) || !sees(pincer2, i, j))
                            continue;
                        if (!grid.getCell(i, j)->candidates[loose]) continue;
                        grid.addInst(encodePos(i, j));
                        grid.addInst(loose);
                        grid.addExec((encodePos(i, j)) << 8 | loose);
                    }
                    if (flag) return;
                }
            }
        }
    }
}