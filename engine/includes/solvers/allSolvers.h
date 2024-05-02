#ifndef ALLSOLVERS_H
#define ALLSOLVERS_H

#include "chain.h"
#include "coloring.h"
#include "fish.h"
#include "sdc.h"
#include "simpleSolvers.h"
#include "singleDigit.h"
#include "subsetFinder.h"
#include "uniqueness.h"
#include "wings.h"

const std::vector<std::function<void(Grid&)>> solvers{
    findNakedSingle,
    findHiddenSingle,
    findLockedCandidates,
    [](Grid& grid) { findNakedSubset(grid, 2); },
    [](Grid& grid) { findNakedSubset(grid, 3); },
    [](Grid& grid) { findNakedSubset(grid, 4); },
    [](Grid& grid) { findHiddenSubset(grid, 2); },
    [](Grid& grid) { findHiddenSubset(grid, 3); },
    [](Grid& grid) { findHiddenSubset(grid, 4); },
    [](Grid& grid) { findFish(grid, 2); },
    [](Grid& grid) { findFish(grid, 3); },
    [](Grid& grid) { findFish(grid, 4); },
    findWWing,
    findXYWing,
    findXYZWing,
    findTurbotFish,
    findEmptyRectangle,
    findSimpleColoring,
    findXChain,
    findXYChain,
    [](Grid& grid) { findFinnedFish(grid, 2); },
    [](Grid& grid) { findFinnedFish(grid, 3); },
    [](Grid& grid) { findFinnedFish(grid, 4); },
    findSDC,
    uniquenessTestType1,
    uniquenessTestType2,
    uniquenessTestType3,
    uniquenessTestType4,
    uniquenessTestType5,
    findHiddenRectangle,
    avoidableRectangle1,
    avoidableRectangle2,
    bivalueUniversalGraveP1,
    findSingleDigitForcing,
    findAIC,
    findNiceLoop};

#endif  // ALLSOLVERS_H