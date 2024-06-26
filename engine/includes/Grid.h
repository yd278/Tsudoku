#ifndef GRID_H
#define GRID_H

#include <_types/_uint16_t.h>

#include <bitset>
#include <random>
#include <string>
#include <vector>
typedef std::vector<uint8_t> Inst;
struct Cell {
    bool given;
    int value;
    std::bitset<9> candidates;
    std::bitset<9> candCouldBe;
    int ans;
    int x, y;
    std::vector<const Cell*> SL;
    Cell() : SL(27), candCouldBe(511) {}
};
struct Exec {
    bool mode;  // true: set, false eliminate;
    std::vector<uint16_t> executees;
    bool operator==(const Exec& other) const {
        return mode == other.mode && executees == other.executees;
    }
};
struct Node {
    int index;
    bool state;
    int target;
    int x, y;
    std::vector<int> edges;
};
class Grid {
   private:
    std::random_device rd;
    unsigned seed;
    std::vector<std::vector<Cell>> grid;

    std::mt19937 gen;

    std::uniform_int_distribution<> distr;
    std::vector<std::vector<std::bitset<9>>> filled;

    std::string compress();
    void checkAndFill(std::string gridPattern);
    bool checkWrongValues();
    bool checkMissingCandidates();
    bool checkWrongCandidates();
    void uniqueness(bool keepIt = false);
    void updateCandCouldBe();
    void updateGraph();
    void updateBiValues();
    void updateStrongLinks();
    void updateFilled();
    std::string toTestString();
    // to generate a board that every value are given
    bool generateFullBoard();
    // to set some value not given and check it still has unique solutions
    void digHoles();

    std::vector<Node> nodes;

    std::vector<std::vector<std::pair<const Cell*, const Cell*>>> strongLinks;
    std::vector<const Cell*> biValues;
    std::vector<std::vector<std::vector<const Cell*>>> biValuesByCands;
    Inst instructions;
    Exec execution;
    bool completed();
    void execute();

   public:
    int checkDifficulty();
    inline void sortExec() {
        sort(execution.executees.begin(), execution.executees.end());
        auto it =
            std::unique(execution.executees.begin(), execution.executees.end());
        execution.executees.resize(
            std::distance(execution.executees.begin(), it));
    }
    inline void addExecToInst() {
        for (auto exe : execution.executees) {
            instructions.push_back(exe >> 8);
            instructions.push_back(exe & 0xff);
        }
    }
    template <typename... Args>
    void addInst(Args... args) {
        (void)std::initializer_list<int>{(instructions.push_back(args), 0)...};
    }
    void addInst(uint8_t inst) { instructions.push_back(inst); }
    inline void addExec(uint16_t exec) { execution.executees.push_back(exec); }
    inline void addExec(uint8_t pos, uint8_t cand) {
        execution.executees.push_back((pos << 8) | cand);
    }

    void addExec(const Cell* cell, uint8_t cand);
    inline void setExec(bool mode) { execution.mode = mode; }
    inline void initInsAndExe() {
        instructions.clear();
        execution.executees.clear();
    }
    inline auto getStrongLinks() const -> const decltype(strongLinks)* {
        return &strongLinks;
    }
    inline auto getFilled() const -> const decltype(filled)* { return &filled; }
    inline auto getBiValues() const -> const decltype(biValues)* {
        return &biValues;
    }
    inline auto getBiValuesByCands() const -> const decltype(biValuesByCands)* {
        return &biValuesByCands;
    }
    inline auto getGraph() const -> const decltype(nodes)* { return &nodes; }
    Grid();
    Grid(int difficulty);
    Grid(std::string gridPattern);
    const Cell* getCell(int x, int y) const;
    const Cell* getCell(std::pair<int, int> pos) const;
    const Cell* getCell(int houseType, int houseID, int cellID) const;
    std::string toString();
    const Inst* nextStep();
    inline const Inst* getInst() const { return &instructions; };
    inline const Exec* getExec() const { return &execution; }
    inline bool emptyExec() const { return execution.executees.empty(); }
};

#endif  // GRID_H