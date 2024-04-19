#include "hiddenSingle.h"

#include "util.h"

std::pair<bool, std::string> findHiddenSingle(const Grid &grid) {
    for (int target = 0; target < 9; target++) {
        for (int houseType = 0; houseType < 3; houseType++) {
            for (int i = 0; i < 9; i++) {
                int cnt = 0;
                std::pair<int, int> res;
                for(int j = 0; j < 9; j++){
                    auto coord = convert(i,j,houseType);
                    auto &c = grid.getCell(coord.first, coord.second);
                    if(c.value==target) break;
                    if(c.candidates[target]) {
                        cnt++;
                        res = coord;
                    }
                }
                if(cnt==1){
                    std::string prompt = "Hidden Single: "+formater(res.first, res.second) + "=" + char(target + '1');
                    return std::make_pair(true, prompt);
                }
            }
        }
    }
    return std::make_pair(false, "");
}