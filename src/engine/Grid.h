#ifndef GRID_H
#define GRID_H

#include <_types/_uint16_t.h>

#include <bitset>
#include <string>
#include <vector>
typedef std::vector<uint8_t> Inst;
struct Cell {
    bool given;
    int value;
    std::bitset<9> candidates;
    int ans;
    int x, y;
};
struct Exec {
    bool mode;  // true: set, false eliminate;
    std::vector<uint16_t> executees;
    bool operator==(const Exec& other) const {
        return mode == other.mode && executees == other.executees;
    }
};
class Grid {
   private:
    std::vector<std::vector<Cell>> grid;
    void checkAndFill(std::string gridPattern);
    bool checkWrongValues();
    bool checkMissingCandidates();
    bool checkWrongCandidates();
    void uniqueness();
    void updateBiValues();
    void updateStrongLinks();

   public:
    std::vector<std::vector<std::pair<const Cell&, const Cell&>>> strongLinks;
    Inst instructions;
    Exec execution;
    std::vector<std::reference_wrapper<const Cell>> biValues;
    inline void addInst(uint8_t inst) { instructions.push_back(inst); }
    inline void addExec(uint16_t exec) { execution.executees.push_back(exec); }
    Grid(int difficulty);
    Grid(std::string gridPattern);
    const Cell& getCell(int x, int y) const;
    const Cell& getCell(std::pair<int, int> pos) const;
    std::string toString();
    Inst& nextStep();
};

#endif  // GRID_H