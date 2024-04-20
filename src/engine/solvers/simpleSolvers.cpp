#include "simpleSolvers.h"

#include <_types/_uint16_t.h>
#include <_types/_uint8_t.h>

#include <iostream>

#include "util.h"

void findNakedSingle(Grid &grid) {
    auto &instruction = grid.instructions;
    auto &execution = grid.execution;
    instruction.clear();
    execution.mode = false;
    execution.executees.clear();
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            auto &c = grid.getCell(i, j);
            if (c.candidates.count() == 1) {
                int tar = 0;
                while (!c.candidates[tar]) tar++;
                instruction.push_back(0x00);
                uint8_t bPos = encodePos(i, j);
                instruction.push_back(bPos);
                instruction.push_back(tar);
                execution.mode = true;
                uint16_t executee = (bPos << 8) | tar;
                execution.executees.push_back(executee);
                return;
            }
        }
    }
}
void findHiddenSingle(Grid &grid) {
    auto &instruction = grid.instructions;
    auto &execution = grid.execution;
    instruction.clear();
    execution.mode = false;
    execution.executees.clear();
    for (int target = 0; target < 9; target++) {
        for (int houseType = 0; houseType < 3; houseType++) {
            for (int i = 0; i < 9; i++) {
                int cnt = 0;
                std::pair<int, int> res;
                for (int j = 0; j < 9; j++) {
                    auto coord = convert(i, j, houseType);
                    auto &c = grid.getCell(coord.first, coord.second);
                    if (c.value == target + 1) break;
                    if (c.candidates[target]) {
                        cnt++;
                        res = coord;
                    }
                }
                if (cnt == 1) {
                    instruction.push_back(0x01);
                    auto bPos = encodePos(res.first, res.second);
                    instruction.push_back(bPos);
                    instruction.push_back(target);
                    execution.mode = true;
                    execution.executees.push_back((bPos << 8) | target);
                    return;
                }
            }
        }
    }
}

void findLockedCandidates(Grid &grid) {
    auto &instruction = grid.instructions;
    auto &execution = grid.execution;
    instruction.clear();
    execution.mode = false;
    execution.executees.clear();
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
                        instruction.push_back(0x10);
                        for (auto &c : intersection) {
                            auto bPos = encodePos(c.get().x, c.get().y);
                            instruction.push_back(bPos);
                            instruction.push_back(target);
                        }
                        execution.mode = false;
                        for (auto &c : lineRemaining) {
                            if (c.get().candidates[target]) {
                                auto bPos = encodePos(c.get().x, c.get().y);
                                instruction.push_back(bPos);
                                instruction.push_back(target);
                                execution.executees.push_back((bPos << 8) |
                                                              target);
                            }
                        }
                        return;
                    }
                    if (!targetIn(target, lineRemaining) &&
                        targetIn(target, boxRemaining)) {
                        instruction.push_back(0x11);
                        for (auto &c : intersection) {
                            auto bPos = encodePos(c.get().x, c.get().y);
                            instruction.push_back(bPos);
                            instruction.push_back(target);
                        }
                        execution.mode = false;
                        for (auto c : boxRemaining) {
                            if (c.get().candidates[target]) {
                                auto bPos = encodePos(c.get().x, c.get().y);
                                instruction.push_back(bPos);
                                instruction.push_back(target);
                                execution.executees.push_back((bPos << 8) |
                                                              target);
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}