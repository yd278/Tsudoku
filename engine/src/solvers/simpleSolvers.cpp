#include "solvers/simpleSolvers.h"

#include "util.h"

void findNakedSingle(Grid &grid) {
    grid.initInsAndExe();
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            auto c = grid.getCell(i, j);
            if (c->candidates.count() == 1) {
                int tar = 0;
                while (!c->candidates[tar]) tar++;
                grid.addInst(0x00);
                uint8_t bPos = encodePos(i, j);
                grid.addInst(bPos);
                grid.addInst(tar);
                grid.setExec(true);
                uint16_t executee = (bPos << 8) | tar;
                grid.addExec(executee);
                return;
            }
        }
    }
}
void findHiddenSingle(Grid &grid) {
    grid.initInsAndExe();
    FOR_ALL(target) {
        for (int houseType = 0; houseType < 3; houseType++) {
            for (int i = 0; i < 9; i++) {
                int cnt = 0;
                std::pair<int, int> res;
                for (int j = 0; j < 9; j++) {
                    auto coord = convert(i, j, houseType);
                    auto c = grid.getCell(coord.first, coord.second);
                    if (c->value == target + 1) break;
                    if (c->candidates[target]) {
                        cnt++;
                        res = coord;
                    }
                }
                if (cnt == 1) {
                    grid.addInst(0x01);
                    auto bPos = encodePos(res.first, res.second);
                    grid.setExec(true);
                    grid.addExec(bPos, target);
                    grid.addExecToInst();
                    return;
                }
            }
        }
    }
}

void findLockedCandidates(Grid &grid) {
    grid.initInsAndExe();
    for (int lineType : {0, 1}) {
        for (int box = 0; box < 9; box++) {
            for (int line = 0; line < 9; line++) {
                if (lineType && (line / 3 != box % 3)) continue;
                if (!lineType && (line / 3 != box / 3)) continue;
                auto res = boxIntersection(box, lineType, line, grid);
                auto &intersection = std::get<0>(res);
                auto &boxRemaining = std::get<1>(res);
                auto &lineRemaining = std::get<2>(res);
                for (int target = 0; target < 9; target++) {
                    if (!targetIn(target, intersection)) continue;
                    if (!targetIn(target, boxRemaining) &&
                        targetIn(target, lineRemaining)) {
                        grid.addInst(0x10);
                        for (auto c : intersection) {
                            auto bPos = encodePos(c->x, c->y);
                            grid.addInst(bPos);
                            grid.addInst(target);
                        }
                        grid.setExec(false);
                        for (auto c : lineRemaining) {
                            if (c->candidates[target]) {
                                auto bPos = encodePos(c->x, c->y);
                                grid.addExec((bPos << 8) | target);
                            }
                        }
                        grid.addExecToInst();
                        return;
                    }
                    if (!targetIn(target, lineRemaining) &&
                        targetIn(target, boxRemaining)) {
                        grid.addInst(0x11);
                        for (auto &c : intersection) {
                            auto bPos = encodePos(c->x, c->y);
                            grid.addInst(bPos);
                            grid.addInst(target);
                        }
                        grid.setExec(false);
                        for (auto c : boxRemaining) {
                            if (c->candidates[target]) {
                                auto bPos = encodePos(c->x, c->y);
                                grid.addExec((bPos << 8) | target);
                            }
                        }
                        grid.addExecToInst();
                        return;
                    }
                }
            }
        }
    }
}