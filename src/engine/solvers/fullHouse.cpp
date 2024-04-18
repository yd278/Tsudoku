#include "fullHouse.h"
#include "util.h"

std::pair<bool, std::string> findFullHouse(const Grid &grid){
    for(int houseType = 0; houseType < 3; houseType++){
        for(int i = 0; i < 9; i++){
            int valCount = 0;
            std::pair<int,int> res;
            char resVal;
            for(int j = 0; j < 9; j++){
                std::pair<int,int> tmp = convert(i, j, houseType);
                auto c = grid.getCell(tmp.first, tmp.second);
                if(c.value!=0){
                    valCount++;
                }else{
                    res = tmp;
                    for(int i = 0; i < 9; i++) if(c.candidates[i]) resVal = i + '1';
                }
            }
            if(valCount==8){
                std::string prompt = "Full House: " + formater(res.first, res.second) + "=" + resVal; 
                return std::make_pair(true, prompt);
            }
        }
    }
    return std::make_pair(false, "");
}