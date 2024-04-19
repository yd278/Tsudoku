#include "nakedSingle.h"
#include "util.h"
std::pair<bool,std::string> findNakedSingle(const Grid &grid){
    for(int i = 0; i < 9; i++){
        for(int j = 0; j < 9; j++){
            auto &c = grid.getCell(i, j);
            if(c.candidates.count()==1){
                int pos = 0;
                while(!c.candidates[pos])pos++;
                std::string prompt = "Naked Single" + formater(i, j)+"=" + char(pos + '1');
                return std::make_pair(true, prompt);
            }
        }
    }
    return std::make_pair(false, "");
}
