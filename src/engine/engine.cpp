#include "Grid.h"

std::string generate(int difficulty){
    Grid grid(difficulty);
    return grid.toString();
}

std::string findNextStep(std::string gridPattern){
    Grid grid(gridPattern);
    return grid.nextStep();
}