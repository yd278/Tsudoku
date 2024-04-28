#include "solvers/coloring.h"

#include <queue>
#include "util.h"

void findSimpleColoring(Grid &grid) {
    FOR_ALL(target) {
        std::vector<const Cell *> nodes;
        std::vector<std::vector<int>> lookup;
        lookup.resize(9);
        for (auto &row : lookup) row.resize(9);
        FOR_ALL(x) FOR_ALL(y) {
            auto cell = grid.getCell(x, y);

            if (cell->candidates[target]) {
                nodes.push_back(cell);
                lookup[cell->x][cell->y] = nodes.size() - 1;
            }
        }
        int n = nodes.size();
        std::vector<int> colored(n);
        std::vector<bool> used(n);
        for (int i = 0; i < n; i++) {
            if (!used[i]) {
                used[i] = true;
                // clear color table
                std::fill(colored.begin(), colored.end(), 0);
                // paint the first cell
                colored[i] = 1;
                // do the bfs
                std::queue<int> q;
                q.push(i);
                while (!q.empty()) {
                    int head = q.front();
                    auto headCell = nodes[head];
                    q.pop();
                    for (auto houseType : {0, 1, 2}) {
                        auto next = headCell->SL[houseType * 9 + target];
                        if (next != nullptr) {
                            int index = lookup[next->x][next->y];
                            if (colored[index] == 0) {
                                used[index] = true;
                                q.push(index);
                                colored[index] = -colored[head];
                            }
                        }
                    }
                }

                // check if elimination exists
                for (int j = 0; j < n; j++) {
                    if (colored[j] != 0) continue;
                    // find an uncolored cell;
                    auto exe = nodes[j];
                    // check if it sees both colors
                    bool seePositive = false;
                    bool seeNegative = false;
                    for (int k = 0; k < n; k++) {
                        if (colored[k] == 0) continue;
                        if (sees(nodes[k], nodes[j])) {
                            if (colored[k] == 1) seePositive = true;
                            if (colored[k] == -1) seeNegative = true;
                        }
                    }
                    if (seePositive && seeNegative) {
                        grid.addExec(exe, target);
                    }
                }
                if (!grid.emptyExec()) {
                    // this coloring does catch something
                    std::vector<const Cell *> positive;
                    std::vector<const Cell *> negative;
                    for (int j = 0; j < n; j++) {
                        if (colored[j] == 1) positive.push_back(nodes[j]);
                        if (colored[j] == -1) negative.push_back(nodes[j]);
                    }
                    grid.addInst(0x70);
                    grid.addInst(target);
                    grid.addInst(positive.size());
                    for (auto c : positive) grid.addInst(encodePos(c));
                    grid.addInst(negative.size());
                    for (auto c : negative) grid.addInst(encodePos(c));
                    grid.addExecToInst();
                    return;
                }
            }
        }
    }
}

void conflictFound(Grid &grid, std::vector<uint8_t> &chain, int houseType,
                   int index, int target) {
    grid.initInsAndExe();
    grid.setExec(false);
    grid.addExec((chain.front() << 8) | target);

    grid.addInst(0x71);
    grid.addInst(target);
    grid.addInst(chain.size());
    for (auto c : chain) grid.addInst(c);
    if (houseType == 2)
        grid.addInst(0xb0 | index);
    else
        grid.addInst(encodeLine(houseType, index));
    grid.addExecToInst();
    return;
}

void findSingleDigitForcing(Grid &grid) {
    FOR_ALL(target) {

        // build virtual grid:
        std::vector<std::vector<bool>> board(9);
        for (auto &row : board) row.resize(9);
        std::vector<std::vector<int>> count(3);
        for (auto &row : count) row.resize(9);
        FOR_ALL(i) FOR_ALL(j) {
            if (grid.getCell(i, j)->candidates[target]) {
                board[i][j] = true;
                count[0][i]++;
                count[1][j]++;
                int box = findBox(i, j);
                count[2][box]++;
            }
        }
        int n = 0;
        for (int i = 0; i < 9; i++)
            if (count[0][i]) n++;

        // select
        FOR_ALL(si) FOR_ALL(sj) {
            if (!board[si][sj]) continue;
            int ci = si;
            int cj = sj;

            auto tmpBoard = board;
            auto tmpCount = count;
            std::vector<uint8_t> chain;
            for (int iter = 0; iter < n; iter++) {
                chain.push_back(encodePos(ci, cj));
                // remove c
                tmpBoard[ci][cj] = false;
                tmpCount[0][ci]--;
                tmpCount[1][cj]--;
                int box = findBox(ci, cj);
                tmpCount[2][box]--;

                // remove cells controlled bt c

                // row
                FOR_ALL(col) {
                    if (tmpBoard[ci][col]) {
                        
                        tmpBoard[ci][col] = false;
                        tmpCount[0][ci]--;
                        tmpCount[1][col]--;
                        if (tmpCount[1][col] == 0) {
                            // conflict found
                            conflictFound(grid, chain, 1, col, target);
                            return;
                        }
                        int oBox = findBox(ci, col);
                        tmpCount[2][oBox]--;
                        if (oBox != box && (tmpCount[2][oBox] == 0)) {
                            // conflict found
                            conflictFound(grid, chain, 2, oBox, target);
                            return;
                        }
                    }
                }
                // col
                FOR_ALL(row) {
                    if (tmpBoard[row][cj]) {
                        tmpBoard[row][cj] = false;
                        tmpCount[0][row]--;
                        if (tmpCount[0][row] == 0) {
                            // conflict found
                            conflictFound(grid, chain, 0, row, target);
                             return;
                        }
                        tmpCount[1][cj]--;

                        int oBox = findBox(row, cj);
                        tmpCount[2][oBox]--;
                        if (oBox != box && (tmpCount[2][oBox] == 0)) {
                            // conflict found
                            conflictFound(grid, chain, 2, oBox, target);
                             return;
                        }
                    }
                }
                // box

                FOR_ALL(index) {
                    auto pos = convert(box, index, 2);
                    
                        
                    if(!tmpBoard[pos.first][pos.second])continue;
                    tmpBoard[pos.first][pos.second] = false;
                    tmpCount[0][pos.first]--;
                    if (tmpCount[0][pos.first] == 0) {
                        // con
                        conflictFound(grid, chain, 0, pos.first,target);
                         return;
                    }
                    tmpCount[1][pos.second]--;
                    if (tmpCount[1][pos.second] == 0) {
                        // c
                        conflictFound(grid, chain, 1, pos.second,target);
                         return;
                    }
                    tmpCount[2][box]--;
                }

                // find next chain node

                bool found = false;
                for (int houseType : {0, 1, 2}) {
                    FOR_ALL(house) {
                        int cnt = 0;

                        FOR_ALL(index) {
                            auto pos = convert(house, index, houseType);
                            if (tmpBoard[pos.first][pos.second]) {
                                cnt++;
                                ci = pos.first;
                                cj = pos.second;
                                if (cnt > 1) break;
                            }
                        }
                        if (cnt == 1) {
                            found = true;
                            break;
                        }
                    }
                    if (found) break;
                }
                if (!found) break;
            }
        }
    }
}