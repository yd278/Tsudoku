#include "solvers/chain.h"

#include <cstdint>
#include <queue>
#include <utility>
#include <vector>

#include "util.h"

void dfs(int cur, int start, const std::vector<Node>* graph,
         std::vector<bool>& inStack, std::vector<int>& path,
         std::vector<std::vector<int>>& paths) {
    inStack[cur] = true;
    path.push_back(cur);
    for (auto next : (*graph)[cur].edges) {
        if (!inStack[next])
            dfs(next, start, graph, inStack, path, paths);
        else if (next == start && path.size() > 1) {
            paths.push_back(path);
        }
    }
    path.pop_back();
    inStack[cur] = false;
};
void makeBiLocal(Grid& grid, const Cell* a, const Cell* b, int t) {
    FOR_ALL(ei) FOR_ALL(ej) {
        auto exe = grid.getCell(ei, ej);
        if (!exe->candidates[t]) continue;
        if (!sees(exe, a) || !sees(exe, b)) continue;
        grid.addExec(exe, t);
    }
}
void findNiceLoop(Grid& grid) {
    auto graph = grid.getGraph();
    std::vector<bool> visited(graph->size());
    std::vector<bool> inStack(graph->size());
    std::vector<int> path;
    std::vector<std::vector<int>> paths;
    for (int i = 0; i < graph->size(); i++) {
        dfs(i, i, graph, inStack, path, paths);
    }
    std::sort(paths.begin(), paths.end(),
              [](const std::vector<int>& a, const std::vector<int>& b) {
                  return a.size() < b.size();
              });
    for (auto path : paths) {
        for (int i = 2; i < path.size(); i += 2) {
            int j = i - 1;
            auto cur = graph->at(path[i]);
            auto nxt = graph->at(path[j]);
            if (cur.target == nxt.target) {
                makeBiLocal(grid, grid.getCell(cur.x, cur.y),
                            grid.getCell(nxt.x, nxt.y), cur.target);
            } else {
                auto exe = grid.getCell(cur.x, cur.y);
                FOR_ALL(cand) {
                    if (cand == cur.target || cand == nxt.target) continue;
                    if (!exe->candidates[cand]) continue;
                    grid.addExec(exe, cand);
                }
            }
        }
        auto cur = graph->at(path.front());
        auto nxt = graph->at(path.back());
        if (cur.target == nxt.target) {
            makeBiLocal(grid, grid.getCell(cur.x, cur.y),
                        grid.getCell(nxt.x, nxt.y), cur.target);
        } else {
            auto exe = grid.getCell(cur.x, cur.y);
            FOR_ALL(cand) {
                if (cand == cur.target || cand == nxt.target) continue;
                if (!exe->candidates[cand]) continue;
                grid.addExec(exe, cand);
            }
        }

        if (!grid.emptyExec()) {
            grid.addInst(0xc2);
            grid.addInst(path.size());
            for (int p : path) {
                auto node = graph->at(p);
                grid.addInst(encodePos(node.x, node.y), node.target);
            }
            grid.sortExec();
            grid.addExecToInst();
            return;
        }
    }
}
// ADD the chain int AIC
int checkAIC(
    Grid& grid, const std::vector<Node>* graph, int head, int tail,
    std::vector<int>& pr,
    std::vector<std::pair<std::vector<uint16_t>, std::vector<uint16_t>>>&
        AICs) {
    auto& nx = graph->at(head);
    auto& ny = graph->at(tail);
    if (ny.target == nx.target) {
        // possible type 1;
        if (sees(nx.x, nx.y, ny.x, ny.y))
            return 0;  // may have more exes, leave to CNL
        std::vector<uint16_t> exes;
        FOR_ALL(ei) FOR_ALL(ej) {
            auto exe = grid.getCell(ei, ej);
            if (!exe->candidates[ny.target]) continue;
            if (sees(exe, nx.x, nx.y) && sees(exe, ny.x, ny.y)) {
                exes.push_back((encodePos(exe) << 8) | ny.target);
            }
        }
        if (!exes.empty()) {
            int cur = tail;
            std::vector<uint16_t> AICCells;
            while (cur != 0) {
                auto AICNode = graph->at(cur);
                AICCells.push_back((encodePos(AICNode.x, AICNode.y) << 8) |
                                   AICNode.target);
                cur = pr[cur];
            }
            auto AICEntity =
                std::make_pair(std::move(AICCells), std::move(exes));
            AICs.push_back(std::move(AICEntity));
            return 1;
        }

    } else if (sees(nx.x, nx.y, ny.x, ny.y)) {
        // possible type 2;
        if (nx.target == ny.target) return 0;

        std::vector<uint16_t> exes;
        auto cx = grid.getCell(nx.x, nx.y);
        if (cx->candidates[ny.target]) {
            exes.push_back((encodePos(cx) << 8) | ny.target);
        }
        auto cy = grid.getCell(ny.x, ny.y);
        if (cy->candidates[nx.target]) {
            exes.push_back((encodePos(cy) << 8) | nx.target);
        }
        if (!exes.empty()) {
            int cur = tail;
            std::vector<uint16_t> AICCells;
            while (cur != 0) {
                auto AICNode = graph->at(cur);
                AICCells.push_back((encodePos(AICNode.x, AICNode.y) << 8) |
                                   AICNode.target);
                cur = pr[cur];
            }
            auto AICEntity =
                std::make_pair(std::move(AICCells), std::move(exes));
            AICs.push_back(std::move(AICEntity));
            return 2;
        }
    }
    return 0;
}

bool bfsFindChain(const std::vector<Node>* graph, int i, int& limit,
                  std::function<bool(int, int)> predicate,
                  std::function<bool(int, std::vector<int>&)> process) {
    std::queue<int> q;
    std::vector<int> dist(graph->size(), INT_MAX);
    std::vector<int> pr(graph->size());
    bool found = false;
    q.push(i);
    dist[i] = 0;
    while (!q.empty()) {
        int h = q.front();
        q.pop();
        if (dist[h] >= limit - 1) {
            break;
        }

        for (auto next : graph->at(h).edges) {
            if (!predicate(h, next)) continue;
            if (dist[next] <= dist[h] + 1) continue;
            q.push(next);

            pr[next] = h;
            dist[next] = dist[h] + 1;
            if (dist[next] % 2 == 1) {
                if (process(next, pr)) {
                    limit = dist[next];
                    found = true;
                    break;
                }
            }
        }

        if (found) break;
    }
    return found;
}

bool cmpAIC(std::pair<std::vector<uint16_t>, std::vector<uint16_t>>& a,
            std::pair<std::vector<uint16_t>, std::vector<uint16_t>>& b) {
    if (a.first.size() < b.first.size()) return true;
    if (a.first.size() == b.first.size() && a.second.size() > b.second.size())
        return true;
    return false;
}
void findAIC(Grid& grid) {
    int limit = INT_MAX;
    auto graph = grid.getGraph();
    std::vector<std::pair<std::vector<uint16_t>, std::vector<uint16_t>>> AICs;
    std::vector<int> AICstates;

    for (int i = 0; i < graph->size() / 2; i++) {
        int AICstate = 0;

        int AICend = -1;
        auto process = [&grid, graph, i, &AICs, &AICstate, &AICend, &AICstates](
                           int next, std::vector<int>& pr) {
            AICstate = checkAIC(grid, graph, i, next, pr, AICs);
            if (AICstate != 0) {
                AICstates.push_back(AICstate);
                AICend = next;
                return true;
            }
            return false;
        };

        bfsFindChain(
            graph, i, limit, [](int h, int next) { return true; }, process);
    }
    if (AICs.empty()) {
        return;
    }
    int res = 0;
    for (int i = 0; i < AICs.size(); i++) {
        if (cmpAIC(AICs[i], AICs[res])) res = i;
    }

    grid.addInst(0xd0 + AICstates[res] - 1);

    auto& chain = AICs[res].first;

    grid.addInst(chain.size());
    for (auto p : chain) {
        grid.addInst(p >> 8, p & 0xf);
    }
    auto exes = AICs[res].second;
    for (auto p : exes) {
        grid.addExec(p);
    }
    grid.sortExec();
    grid.addExecToInst();
    return;
}

void findXChain(Grid& grid) {
    int limit = INT_MAX;

    std::vector<uint8_t> shortestChain;
    std::vector<uint16_t> bestExec;
    int bestTarget;

    auto graph = grid.getGraph();
    for (int i = 0; i < graph->size() / 2; i++) {
        int target = graph->at(i).target;

        auto predicate = [=](int h, int next) {
            return (graph->at(next).target == target);
        };
        auto process = [&shortestChain, &bestExec, graph, i, &grid, &bestTarget,
                        target](int next, std::vector<int>& pr) {
            auto& head = graph->at(i);
            auto& tail = graph->at(next);
            std::vector<uint16_t> exec;
            FOR_ALL(i) FOR_ALL(j) {
                if (sees(head.x, head.y, i, j) && sees(tail.x, tail.y, i, j)) {
                    auto cell = grid.getCell(i, j);
                    if (cell->candidates[head.target]) {
                        exec.push_back((encodePos(cell)) << 8 | head.target);
                    }
                }
            }
            if (exec.empty()) return false;  // nothing to eliminate
            bestExec = std::move(exec);
            std::vector<uint8_t> chain;
            for (int cur = next; cur != 0; cur = pr[cur]) {
                chain.push_back(encodePos(graph->at(cur).x, graph->at(cur).y));
            }
            shortestChain = std::move(chain);
            bestTarget = target;
            return true;
        };

        bfsFindChain(graph, i, limit, predicate, process);
    }
    if (shortestChain.empty()) {
        return;
    }

    grid.addInst(0x60);
    grid.addInst(shortestChain.size());
    for (auto c : shortestChain) {
        grid.addInst(c);
    }
    grid.addInst(bestTarget);

    for (auto e : bestExec) grid.addExec(e);
    grid.sortExec();
    grid.addExecToInst();
    return;
}

void findXYChain(Grid& grid) {
    int limit = INT_MAX;
    auto graph = grid.getGraph();

    std::vector<uint8_t> shortestChain;
    std::vector<uint16_t> bestExec;
    for (int i = 0; i < graph->size() / 2; i++) {
        auto node = graph->at(i);
        auto cell = grid.getCell(node.x, node.y);
        if (cell->candidates.count() != 2) continue;
        // start with a bi-value

        auto predicate = [graph](int h, int next) {
            auto hNode = graph->at(h);
            auto nextNode = graph->at(next);
            if (hNode.state == false) {
                return hNode.x == nextNode.x && hNode.y == nextNode.y;
            } else {
                return hNode.target == nextNode.target;
            }
            return false;
        };

        auto process = [graph, i, &grid, &bestExec, &shortestChain](
                           int next, std::vector<int>& pr) {
            auto endNode = graph->at(next);
            if (endNode.target != graph->at(i).target) return false;
            auto startCell = grid.getCell(graph->at(i).x, graph->at(i).y);
            auto endCell = grid.getCell(endNode.x, endNode.y);

            std::vector<uint16_t> exec;
            FOR_ALL(i) FOR_ALL(j) {
                auto exe = grid.getCell(i, j);
                if (!exe->candidates[endNode.target]) continue;
                if (sees(exe, startCell) && sees(exe, endCell)) {
                    exec.push_back((encodePos(exe) << 8) | endNode.target);
                }
            }
            if (exec.empty()) return false;
            bestExec = std::move(exec);
            std::vector<uint8_t> chain;
            for (auto cur = next; cur != 0; cur = pr[cur]) {
                chain.push_back(encodePos(graph->at(cur).x, graph->at(cur).y));
                chain.push_back(graph->at(cur).target);
            }
            shortestChain = std::move(chain);
            return true;
        };

        bfsFindChain(graph, i, limit, predicate, process);
    }
    if (shortestChain.empty()) {
        return;
    }

    grid.addInst(0x61);
    grid.addInst(shortestChain.size() / 2);
    for (auto c : shortestChain) {
        grid.addInst(c);
    }

    for (auto e : bestExec) grid.addExec(e);
    grid.sortExec();
    grid.addExecToInst();
    return;
}