#include "util.h"

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

std::string formater(int x,int y){
    std::string s = "(";
    s+= (x+'1');
    s+= ",";
    s+= (y+'1');
    s+=(")");
    return s;
}