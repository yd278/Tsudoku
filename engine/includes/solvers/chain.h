#ifndef CHAIN_H
#define CHAIN_H

#include <functional>
#include "Grid.h"
void findXChain(Grid &grid);
void findXYChain(Grid &grid);
void findAIC(Grid &grid);
void findNiceLoop(Grid &grid);

// the counter-part step only allowed to pass once;

void dfs(int cur,                     // current node index
         std::vector<Node> * graph,  //graph
         std::vector<bool> &visited,  // visited[i]: node[i] is visited
         std::vector<bool> &inStack, // node[i] is in the stack, if encountered a node in stack, a loop is found
         std::vector<int> &path,      // path from search root to current node
         std::vector<std::vector<int> >& paths,// keep all the paths
         bool reversed                    // counter-part step already in path
);


//after we get the shortest loop
// go through the path see if exists adjacent two nodes are counter-parts of each other;
//if yes, its' a DNL, start by the second of the counter-parts and close the loop
// if no, it's a CNL, for each pair in a weak link (from true to false), eliminate cells see both of them
// if shortest path contains nothing to execute??
// solved by keeping all paths
// can paths redundant to each other?
// no because it will be marked visited in the dfs process


// 
void findLoop(std::vector<int> &path,std::vector<std::vector<int> >& paths, int start);
bool bfs(const std::vector<Node>* graph, int i,int limit, std::function<bool(int)>predicate, std::function<bool(int)>process);
#endif  // CHAIN_H