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


uint8_t encodePos(std::pair<int,int> pos){
    return encodePos(pos.first,pos.second);
}

std::tuple<std::vector<std::reference_wrapper<const Cell>>,
           std::vector<std::reference_wrapper<const Cell>>,
           std::vector<std::reference_wrapper<const Cell>>>
boxIntersection(int boxIndex, int lineType, int lineIndex, const Grid &grid) {
    if (lineType) {
        if (lineIndex / 3 != boxIndex % 3)
            return std::make_tuple(
                std::vector<std::reference_wrapper<const Cell>>(),
                std::vector<std::reference_wrapper<const Cell>>(),
                std::vector<std::reference_wrapper<const Cell>>());
    } else {
        if (lineIndex / 3 != boxIndex / 3)
            return std::make_tuple(
                std::vector<std::reference_wrapper<const Cell>>(),
                std::vector<std::reference_wrapper<const Cell>>(),
                std::vector<std::reference_wrapper<const Cell>>());
    }
    std::vector<std::reference_wrapper<const Cell>> intersect;
    std::vector<std::reference_wrapper<const Cell>> boxRemaining;
    std::vector<std::reference_wrapper<const Cell>> lineRemaining;

    int boxSubIndex = lineType ? (boxIndex / 3) : (boxIndex % 3);
    for (int seg = 0; seg < 3; seg++) {
        for (int i = 0; i < 3; i++) {
            auto coord = convert(lineIndex, i + seg * 3, lineType);
            if (seg == boxSubIndex)
                intersect.push_back(grid.getCell(coord.first, coord.second));
            else
                lineRemaining.push_back(
                    grid.getCell(coord.first, coord.second));
        }
    }
    for (int i = 0; i < 9; i++) {
        auto coord = convert(boxIndex, i, 2);
        int comp = lineType ? coord.second : coord.first;
        if (comp == lineIndex) continue;
        boxRemaining.push_back(grid.getCell(coord.first, coord.second));
    }

    return std::make_tuple(intersect, boxRemaining, lineRemaining);
}

bool targetIn(int target,
              std::vector<std::reference_wrapper<const Cell>> cells) {
    for (auto &cell : cells) {
        if (cell.get().candidates[target]) return true;
    }
    return false;
}

uint8_t encodePos(int x, int y){
    return x<<4|y;
}