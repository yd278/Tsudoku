#include "solvers/coloring.h"

#include <set>

#include "util.h"
void findCover(const Cell *cell, std::vector<std::bitset<9>> &covered) {
    covered.resize(9);
    covered[cell->x].set();
    FOR_ALL(i) { covered[i].set(cell->y); }
    int box = findBox(cell);
    FOR_ALL(i) {
        auto pos = convert(box, i, 2);
        covered[pos.first].set(pos.second);
    }
}
const Cell *dfs(Grid &grid, const Cell *cur, int flip,
                std::vector<std::bitset<9>> &covered, int target,
                std::set<const Cell *> &used,
                std::vector<const Cell *> &stack) {
    if (flip) {
        std::vector<std::bitset<9>> currentCover(9);
        findCover(cur, currentCover);
        int cnt = 0;
        FOR_ALL(i) { cnt += (covered[i] | currentCover[i]).count(); }
        if (cnt) return cur;
    }
    for (int i : {0, 1, 2}) {
        auto next = cur->SL[i * 9 + target];
        if (used.find(next) == used.end()) {
            used.insert(next);
            stack.push_back(next);
            auto tmp = dfs(grid, next, 1 - flip, covered, target, used, stack);
            if (tmp != nullptr) return tmp;
            stack.pop_back();
        }
    }
    return nullptr;
}
void findSimpleColoring(Grid &grid) {
    std::vector<const Cell *> stack;
    stack.reserve(54);
    FOR_ALL(target) {
        for (auto pair : (*grid.getStrongLinks())[target]) {
            for (auto p1 : {pair.first, pair.second}) {
                std::vector<std::bitset<9>> covered;
                findCover(p1, covered);
                std::set<const Cell *> used;
                stack.push_back(p1);
                auto tmp = dfs(grid, p1, 0, covered, target, used, stack);
                if (tmp) {
                    grid.addInst(0x70);
                    grid.addInst(stack.size());
                    for(auto c : stack){
                        grid.addInst(encodePos(c));
                    }

                    FOR_ALL(ei) FOR_ALL(ej){
                        auto exec = grid.getCell(ei,ej);
                        if(exec->candidates[target]){
                            if(sees(exec,p1) && sees(exec,tmp)){
                                grid.addExec(exec,target);
                            }
                        }
                    }
                    grid.addExecToInst();
                }
                stack.pop_back();
            }
        }
    }
}