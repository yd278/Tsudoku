#ifndef UTIL_H
#define UTIL_H

#include <string>
#include <utility>

#include "Grid.h"

#define FOR_ALL(var) for (int var = 0; var < 9; var++)

uint8_t encodePos(int x, int y);
uint8_t encodePos(std::pair<int, int> pos);
uint8_t encodePos(const Cell *cell);

std::pair<int, int> convert(int house, int index, int houseType);
// intersection, boxRemaining, lineRemaining
std::tuple<std::vector<const Cell *>, std::vector<const Cell *>,
           std::vector<const Cell *>>
boxIntersection(int boxIndex, int lineType, int lineIndex, const Grid &grid);

bool targetIn(int target, std::vector<const Cell *> cells);

bool sees(int x1, int y1, int x2, int y2);
bool sees(const Cell *cell, int x, int y);

bool sees(const Cell *cell1, const Cell *cell2);

#endif  // UTIL_H