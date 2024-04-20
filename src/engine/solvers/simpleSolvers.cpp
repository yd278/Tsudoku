#include "simpleSolvers.h"
#include <_types/_uint16_t.h>
#include <_types/_uint8_t.h>

#include <iostream>

#include "util.h"


void findNakedSingle(Grid &grid) {
    auto &instruction =  grid.instructions;
    auto &execution = grid.execution;
    instruction.clear();
    execution.mode=false;
    execution.executees.clear();
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            auto &c = grid.getCell(i, j);
            if (c.candidates.count() == 1) {
                int tar = 0;
                while (!c.candidates[tar]) tar++;
                instruction.push_back(0x00);
                uint8_t bPos = i<<4|j;
                uint8_t bTar = tar;
                instruction.push_back(bPos);
                instruction.push_back(bTar);
                execution.mode = true;
                uint16_t executee = (bPos<<8) | bTar;
                execution.executees.push_back(executee);
            }
        }
    }
}
std::pair<bool, std::string> findHiddenSingle(const Grid &grid) {
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
                    std::string prompt =
                        "Hidden Single: " + formater(res.first, res.second) +
                        "=" + char(target + '1');
                    return std::make_pair(true, prompt);
                }
            }
        }
    }
    return std::make_pair(false, "");
}

std::pair<bool, std::string> findLockedCandidates(const Grid &grid) {
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
                        std::string prompt =
                            "Locked Candidates Type 1 (Pointing): ";
                        std::vector<std::reference_wrapper<const Cell>> toEli;
                        for (auto &c : lineRemaining) {
                            if (c.get().candidates[target]) toEli.push_back(c);
                        }
                        prompt += formater(toEli);
                        prompt += "!=";
                        prompt += char(target + '1');
                        return std::make_pair(true, prompt);
                    }
                    if (!targetIn(target, lineRemaining) &&
                        targetIn(target, boxRemaining)) {
                        std::string prompt =
                            "Locked Candidates Type 2 (Claiming): ";
                        std::vector<
                            std::vector<std::reference_wrapper<const Cell>>>
                            toEli(3);
                        for (auto c : boxRemaining) {
                            if (c.get().candidates[target]) {
                                toEli[c.get().x % 3].push_back(c);
                            }
                        }
                        prompt += formater(toEli);
                        prompt += "!=";
                        prompt += char(target + '1');
                        return std::make_pair(true, prompt);
                    }
                }
            }
        }
    }
    return std::make_pair(false, "");
}