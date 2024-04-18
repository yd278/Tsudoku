#include <stack>
#include <string>
#include <vector>

std::string solve(std::string);

class DLX {
    int n, m;
    int tail;
    std::vector<int> first, size;
    std::vector<int> L, R, U, D;
    std::vector<int> col, row;
    std::stack<int> stk;

   public:
    std::vector<std::vector<int>> ans;
    bool flag;
    DLX(int r, int c);

    void insert(int r, int c);

    void remove(int pos);

    void recover(int pos);
    void dance(int dep);
};

int getId(int row, int col, int num);
void init(DLX &dlx, std::string &puzzle);
std::string format(DLX &dlx);
