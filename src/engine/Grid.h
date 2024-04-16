#include <string>
class Grid{
    public:
    Grid(int difficulty);
    Grid(std::string gridPattern);
    std::string toString();
    std::string nextStep();
};