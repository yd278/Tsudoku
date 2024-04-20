#include "subsetFinder.h"

#include "util.h"

bool findNakedSubsetRec(
    Grid &grid, int cnt, int dep, int first, int houseType, int houseID,
    std::vector< std::bitset<9>> &candidateList,
    std::bitset<9> &options, std::bitset<9> acc,
    std::vector<int> &selectedCell) {
    if (dep == cnt) {
        if (acc.count() == cnt) {
            grid.instructions.push_back(0x30 + cnt - 2);
            grid.execution.mode = false;
            // make a list of candidates form the naked subset
            std::vector<int> selectedCandidates;
            for (int i = 0; i < 9; i++) {
                if (acc[i]) {
                    selectedCandidates.push_back(i);
                }
            }

            for (int i = 0; i < cnt; i++) {
                int cellID = selectedCell[i];
                auto pos = convert(houseID, cellID, houseType);
                grid.instructions.push_back(encodePos(pos));
                grid.instructions.push_back(selectedCandidates[i]);
            }

            int ptr = 0;
            for (int i = 0; i < 9; i++) {
                if (i == selectedCell[ptr]) {
                    ptr++;
                    continue;
                } else {
                    auto pos = convert(houseID, i, houseType);
                    auto toEli =
                        acc | grid.getCell(pos.first, pos.second).candidates;
                    for (int j = 0; j < 9; j++) {
                        if (toEli[j]) {
                            grid.instructions.push_back(encodePos(pos));
                            grid.instructions.push_back(j);
                            grid.execution.executees.push_back(
                                (encodePos(pos) << 8) | j);
                        }
                    }
                }
            }
            return true;
        }
        return false;
    }
    for (int i = first; i < 9; i++) {
        if (options[i]) {
            selectedCell.push_back(i);
            if (findNakedSubsetRec(grid, cnt, dep + 1, i + 1, houseType,
                                   houseID, candidateList, options,
                                   acc | candidateList[i], selectedCell))
                return true;
            ;
            selectedCell.pop_back();
        }
    }
    return false;
}
void findNakedSubset(Grid &grid, int cnt) {
    grid.execution.mode = false;
    grid.execution.executees.clear();
    grid.instructions.clear();
    for (int houseType = 0; houseType < 3; houseType++) {
        for (int i = 0; i < 9; i++) {
            std::vector<std::bitset<9>> candidateList(9);
            std::bitset<9> options;
            for (int j = 0; j < 9; j++) {
                auto pos = convert(i, j, houseType);
                auto c = grid.getCell(pos.first, pos.second);
                if (c.value == 0) {
                    options.set(j);
                    candidateList[i] =
                        grid.getCell(pos.first, pos.second).candidates;
                }
            }
            if (options.size() <= cnt) continue;
            std::vector<int> selectCell;
            if (findNakedSubsetRec(grid, cnt, 0, 0, houseType, i, candidateList,
                                   options, std::bitset<9>(), selectCell))
                return;
        }
    }
}

bool findHiddenSubsetRec(Grid &grid, int cnt, int dep, int first, int houseType,
                         int houseID, std::vector<std::bitset<9>> &cellList,
                         std::bitset<9> &options, std::bitset<9> acc,
                         std::vector<int> selectCandidates) {
    if (dep == cnt) {
        if (acc.count() == cnt) {
            grid.instructions.push_back(0x34 + cnt - 2);
            grid.execution.mode = false;
            // make a list of select cells id in the hidden subset

            std::vector<int> selectedCells;
            for (int i = 0; i < 9; i++) {
                if (acc[i]) {
                    selectedCells.push_back(i);
                }
            }

            for (int i = 0; i < cnt; i++) {
                int cellID = selectedCells[i];
                auto pos = convert(houseID, cellID, houseType);
                grid.instructions.push_back(encodePos(pos));
                grid.instructions.push_back(selectCandidates[i]);
            }

            for (int i : selectedCells) {
                int ptr = 0;
                for (int j = 0; j < 9; j++) {
                    if (j == selectCandidates[ptr]) {
                        ptr++;
                        continue;
                    }
                    auto pos = convert(houseID, i, houseType);
                    auto c = grid.getCell(pos.first, pos.second);
                    if (c.candidates[j]) {
                        grid.instructions.push_back(encodePos(pos));
                        grid.instructions.push_back(j);
                        grid.execution.executees.push_back(
                            (encodePos(pos) << 8) | j);
                    }
                }
            }

            return true;
        }
        return false;
    }
    for (int i = first; i < 9; i++) {
        if (options[i]) {
            selectCandidates.push_back(i);
            if (findHiddenSubsetRec(grid, cnt, dep + 1, i + 1, houseType,
                                    houseID, cellList, options,
                                    acc | cellList[i], selectCandidates))
                return true;
            ;
            selectCandidates.pop_back();
        }
    }
    return false;
}
void findHiddenSubset(Grid &grid, int cnt) {
    grid.execution.mode = false;
    grid.execution.executees.clear();
    grid.instructions.clear();
    for (int houseType = 0; houseType < 3; houseType++) {
        for (int i = 0; i < 9; i++) {
            std::vector<std::bitset<9>> cellList;
            std::bitset<9> options("111111111");
            for (int j = 0; j < 9; j++) {
                auto pos = convert(i, j, houseType);
                auto c = grid.getCell(pos.first, pos.second);
                for (int tar = 0; tar < 9; tar++) {
                    if (c.candidates[tar]) cellList[tar][j] = true;
                    options[tar] = false;
                }
            }
            std::vector<int> selectCandidates;
            if (findHiddenSubsetRec(grid, cnt, 0, 0, houseType, i, cellList,
                                    options, std::bitset<9>(),
                                    selectCandidates))
                return;
        }
    }
}