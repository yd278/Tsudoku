#ifndef FULLHOUSE_H
#define FULLHOUSE_H

#include "Grid.h"
std:: pair<bool,std::string> findFullHouse(const Grid &grid);
std::pair<bool,std::string> findNakedSingle(const Grid &grid);
std::pair<bool,std::string> findHiddenSingle(const Grid &grid);
std::pair<bool,std::string> findLockedCandidates(const Grid &grid);

#endif // FULLHOUSE_H