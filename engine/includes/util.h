#ifndef UTIL_H
#define UTIL_H

#include <iostream>
#include <utility>

#include "Grid.h"

inline void debugLog() {}
template <typename First, typename... Rest>

void debugLog(First &&first, Rest &&...rest) {
    std::cout << std::forward<First>(first);
    debugLog(std::forward<Rest>(rest)...);
}

#define FOR_ALL(var) for (int var = 0; var < 9; var++)

uint16_t encodeExe(const Cell * cell, int target);
uint8_t encodePos(int x, int y);
uint8_t encodePos(std::pair<int, int> pos);
uint8_t encodePos(const Cell *cell);
uint8_t encodeLine(int lineType, int lineNumber);
inline int findBox(int x, int y) { return (x / 3) * 3 + (y / 3); }

int findBox(const Cell *cell);
std::pair<int, int> convert(int house, int index, int houseType);
// intersection, boxRemaining, lineRemaining
std::tuple<std::vector<const Cell *>, std::vector<const Cell *>,
           std::vector<const Cell *>>
boxIntersection(int boxIndex, int lineType, int lineIndex, const Grid &grid);

bool targetIn(int target, std::vector<const Cell *> cells);

bool sees(int x1, int y1, int x2, int y2);
bool sees(const Cell *cell, int x, int y);

bool sees(const Cell *cell1, const Cell *cell2);
bool cmp(const Cell *cell1, const Cell *cell2);

#endif  // UTIL_H