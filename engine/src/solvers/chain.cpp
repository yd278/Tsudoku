#include "solvers/chain.h"

#include <queue>
#include <vector>
#include "util.h"

void findLoop(std::vector<int>& path, std::vector<std::vector<int>>& paths,
              int start) {
    auto it = std::find(path.begin(), path.end(), start);
    if (it != path.end()) {
        std::vector<int> segment(it, path.end());
        paths.push_back(std::move(segment));
    }
}

void dfs(int cur, const std::vector<Node>* graph, std::vector<bool>& visited,
         std::vector<bool>& inStack, std::vector<int>& path,
         std::vector<std::vector<int>>& paths, bool reversed) {
    visited[cur] = true;
    inStack[cur] = true;
    path.push_back(cur);
    for (auto next : (*graph)[cur].edges) {
        if (inStack[next]) {
            // loop found
            findLoop(path, paths, next);
            return;
        }
        if (visited[next]) return;

        dfs(next, graph, visited, inStack, path, paths, reversed);
    }
    if (!reversed) {
        int cnt = graph->size() / 2;
        int nxt = cur >= cnt ? cur - cnt : cur + cnt;
        if (inStack[nxt]) {
            // loop found
            findLoop(path, paths, nxt);
            return;
        }
        dfs(nxt, graph, visited, inStack, path, paths, true);
    }
    path.pop_back();
    inStack[cur] = false;
    return;
};

void findNiceLoop(Grid& grid) {
    auto graph = grid.getGraph();
    std::vector<bool> visited(graph->size());
    std::vector<bool> inStack(graph->size());
    std::vector<int> path;
    std::vector<std::vector<int>> paths;
    for (int i = 0; i < graph->size(); i++) {
        if (!visited[i]) {
            dfs(i, graph, visited, inStack, path, paths, false);
        }
    }
    std::sort(paths.begin(), paths.end(),
              [](const std::vector<int>& a, const std::vector<int>& b) {
                  return a.size() < b.size();
              });
    for (auto path : paths) {
        // check nice or disco;
        for (int i = 0; i < path.size(); i++) {
            int j = i+1;
            if(j==path.size())j=0;
            if(abs(path[i] - path[j])==graph->size()/2){
                //disco
                // j cannot be true;
                auto n = graph->at(j);
                int x = n.x;
                int y = n.y;
                int target = n.target;
                int mode = n.state;
                grid.initInsAndExe();
                grid.addExec(encodePos(x,y),target);
                grid.setExec(!mode);

                //TODO: set instructions according to protocol
                return;
            }   
        }
        //TODO: set up execs for a CNL:
        //that's a CNL
        //replace every weak link by a strong link
        // same target different cell: eliminate all the same target in cells sees both;
        // same cell different target: eliminate all other candidates;
    }
}
// deal with exec too;
int checkAIC(Grid& grid, const std::vector<Node>* graph, int x, int y){
    //DEBUG
    auto node = graph->at(x);
    auto tail = graph->at(y);
    //END DEBUG
    grid.initInsAndExe();
    grid.setExec(false);
    auto &nx = graph->at(x);
    auto &ny = graph->at(y);
    if(ny.target==nx.target){
        //possible type 1;
        if(sees(nx.x,nx.y,ny.x,ny.y))return 0; // may have more exes, leave to CNL
        FOR_ALL(ei) FOR_ALL(ej){
            auto exe = grid.getCell(ei,ej);
            if(!exe->candidates[ny.target]) continue;
            if(sees(exe,nx.x,nx.y) && sees(exe,ny.x,ny.y)){
                grid.addExec(exe,ny.target);
            }
        }
        if(!grid.emptyExec()) return 1;
    }
    else if(sees(nx.x,nx.y,ny.x,ny.y)){
        //possible type 2;
        if(nx.target==ny.target) return 0;
        auto cx = grid.getCell(nx.x,nx.y);
        if(cx->candidates[ny.target]){
            grid.addExec(cx,ny.target);
        }
        auto cy = grid.getCell(ny.x,ny.y);
        if(cy->candidates[nx.target]){
            grid.addExec(cy,nx.target);
        }
        if(!grid.emptyExec()) return 2;
    }
    return 0;
}

void findAIC(Grid& grid){
    int limit = INT_MAX;
    auto graph = grid.getGraph();
    for(int i = 0; i < graph->size()/2; i++){
        std::queue<int> q;
        std::vector<int> dist(graph->size(),INT_MAX);
        std::vector<int> pr(graph->size());
        dist[i] = 0;
        q.push(i);
        while(!q.empty()){
            int h = q.front();

            q.pop();
            for(auto next : graph->at(h).edges){
                if(dist[next] > dist[h]+1){
                    q.push(next);
                    pr[next] = h;
                    dist[next] = dist[h]+1;
                    if(dist[next]%2==1){
                        int tmp = checkAIC(grid, graph, i, next);
                        if(tmp!=0){
                            grid.addInst(0xf0+tmp-1);
                            grid.addInst(dist[next]+1);
                            for(int cur = next; cur != 0; cur = pr[cur]){
                                auto &n = graph->at(cur);
                                grid.addInst(encodePos(n.x,n.y), n.target);
                            }
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