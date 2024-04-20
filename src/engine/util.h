#ifndef UTIL_H
#define UTIL_H

#include <string>
#include <utility>

#include "Grid.h"

uint8_t encodePos(int x, int y);
uint8_t encodePos(std::pair<int,int> pos);

std::pair<int, int> convert(int house, int index, int houseType);
//intersection, boxRemaining, lineRemaining
std::tuple<std::vector<std::reference_wrapper<const Cell>>,
           std::vector<std::reference_wrapper<const Cell>>,
           std::vector<std::reference_wrapper<const Cell>>>
boxIntersection(int boxIndex, int lineType, int lineIndex, const Grid &grid);

bool targetIn(int target, std::vector<std::reference_wrapper<const Cell>> cells);

#endif  // UTIL_H