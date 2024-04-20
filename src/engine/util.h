#ifndef UTIL_H
#define UTIL_H

#include <string>
#include <utility>

#include "Grid.h"

uint8_t encodePos(int x, int y);

std::pair<int, int> convert(int house, int index, int houseType);
std::string formater(int x, int y);
std::string formater(const Cell &c);
std::string formater(std::vector<std::reference_wrapper<const Cell>> line);
std::string formater(std::vector<std::vector<std::reference_wrapper<const Cell>>> lines);
//intersection, boxRemaining, lineRemaining
std::tuple<std::vector<std::reference_wrapper<const Cell>>,
           std::vector<std::reference_wrapper<const Cell>>,
           std::vector<std::reference_wrapper<const Cell>>>
boxIntersection(int boxIndex, int lineType, int lineIndex, const Grid &grid);

bool targetIn(int target, std::vector<std::reference_wrapper<const Cell>> cells);

#endif  // UTIL_H