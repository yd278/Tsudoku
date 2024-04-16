#include <bitset>
#include <vector>
struct Cell {
    bool given;
    int value;
    std::bitset<9> candidates;
    int ans;
};
class Grid {
   private:
    std::vector<std::vector<Cell>> grid;
    void checkAndFill(std::string gridPattern) {
        if (gridPattern.length() != 972) {
            throw std::invalid_argument("Invalid Pattern: wrong length");
        }
        // initialize grid
        grid.resize(9);
        for (auto &row : grid) row.resize(9);

        // sanity check for each cell while filling in
        for (int i = 0; i < 81; i++) {
            Cell &cur = grid[i / 9][i % 9];
            std::string curCell = gridPattern.substr(i * 12, 12);

            // given flag
            if (curCell[0] != '0' && curCell[0] != '1')
                throw std::invalid_argument(
                    "Invalid Pattern: invalid given flag");
            cur.given = curCell[0] == '1';

            // value
            if (curCell[1] < '0' || curCell[1] > '9')
                throw std::invalid_argument("Invalid Pattern: invalid value");
            cur.value = curCell[1] - '0';

            // candidates
            if (cur.value == 0) {
                for (int j = 0; j < 9; j++) {
                    if (curCell[j + 2] != '0' && curCell[j + 2] != '1')
                        throw std::invalid_argument(
                            "Invalid Pattern: invalid candidates");
                    cur.candidates[j] = curCell[j + 2] == '1';
                }
            } else {
                for (int j = 0; j < 9; j++) {
                    if (curCell[j + 2] != '0')
                        throw std::invalid_argument(
                            "Invalid Pattern: invalid candidates");
                }
            }

            // answer
            if (curCell[11] < '0' || curCell[11] > '9')
                throw std::invalid_argument("Invalid Pattern: invalid answer");
            cur.ans = curCell[11] - '0';
        }
    }

    bool checkWrongValues(){
        for(int i = 0; i < 9; i++){
            for(int j = 0; j < 9; j++){
                if(grid[i][j].value!=0 &&grid[i][j].value != grid[i][j].ans) return false;
            }
        }
        return true;
    }


    bool checkMissingCandidates(){
        for(int i = 0; i < 9; i++){
            for(int j = 0; j < 9; j++){
                if(grid[i][j].value==0){
                    if(!grid[i][j].candidates[grid[i][j].ans-1]) return false;
                }
            }
        }
        return true;
    }
    // houseType: 0-rows, 1-columns, 2-boxes
    std::pair<int, int> convert(int house, int index, int houseType) {
        if (houseType == 0) {
            return std::make_pair(house, index);
        }
        if (houseType == 1) {
            return std::make_pair(index, house);
        }
        if (houseType == 2) {
            // top left of the box:
            int TLi = house / 3 * 3;
            int TLj = house % 3 * 3;

            // offset
            int offsetI = index / 3;
            int offsetJ = index % 3;
            return std::make_pair(TLi + offsetI, TLj + offsetJ);
        }
        return std::make_pair(-1, -1);
    }
    
    bool checkWrongCandidates() {
        for (int houseType = 0; houseType < 3; houseType++) {
            for (int i = 0; i < 9; i++) {
                std::bitset<9> candidatesUnion;
                for (int j = 0; j < 9; j++) {
                    auto coordinate = convert(i, j, houseType);
                    Cell &cell = grid[coordinate.first][coordinate.second];

                    if (grid[i][j].value == 0) {
                        candidatesUnion =
                            candidatesUnion | grid[i][j].candidates;
                    } else {
                        if (candidatesUnion[grid[i][j].value - 1]) return false;
                    }
                }
            }
        }
        return true;
    }

    bool uniqueness(){
        //TODO: use DLX to determine if the solution is unique
        return true;
    }

   public:
    Grid(std::string gridPattern) {
        try {
            checkAndFill(gridPattern);
        } catch (const std::invalid_argument &e) {
            throw;
        }
        //TODO:  uniqueness check and sanity check
        // do the uniqueness check first so we can check the ans from the pattern is correct or not
    }

    Grid(int difficulty) {
        grid.resize(9);
        for (auto &row : grid) row.resize(9);

        // TODO: generate sudoku of given difficulty
    }

    std::string toString(){
        std::string res = "";
        for(int i = 0; i < 9; i++){
            for(int j = 0; j < 9; j++){
                res += grid[i][j].value + '0';
            }
        }
        for(int i = 0; i < 9; i++){
            for(int j = 0; j < 9; j++){
                res += grid[i][j].ans + '0';
            }
        }
        return res;
    }

    std::string nextStep(){
        //TODO: call solvers in sequence from easy to hard;
        //once find a solution, return it;
        return "";
    }
};