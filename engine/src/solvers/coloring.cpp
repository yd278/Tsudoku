#include "solvers/coloring.h"

#include <iostream>
#include <queue>
#include <set>

#include "util.h"

void log() {}

template <typename First, typename... Rest>
void log(First &&first, Rest &&...rest) {
    std::cout << std::forward<First>(first);
    log(std::forward<Rest>(rest)...);
}

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