#ifndef GRID_H
#define GRID_H

#include <bitset>
#include <string>
#include <vector>

struct Cell {
    bool given;
    int value;
    std::bitset<9> candidates;
    int ans;
    int x,y;
};
class Grid {
   private:
    std::vector<std::vector<Cell>> grid;
    void checkAndFill(std::string gridPattern);
    bool checkWrongValues();
    bool checkMissingCandidates();
    bool checkWrongCandidates();
    void uniqueness();

   public:
    
    Grid(int difficulty);
    Grid(std::string gridPattern);
    const Cell & getCell(int x, int y) const;
    std::string toString();
    std::string nextStep();
};

#endif // GRID_H