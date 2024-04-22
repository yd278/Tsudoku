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

uint8_t encodePos(const Cell* cell) { return encodePos(cell->x, cell->y); }

bool sees(int x1, int y1, int x2, int y2) {
    if (x1 == x2 || y1 == y2) return true;
    if (x1 / 3 == x2 / 3 && y1 / 3 == y2 / 3) return true;
    return false;
}

bool sees(const Cell* cell, int x, int y) {
    return sees(cell->x, cell->y, x, y);
}

uint8_t encodePos(std::pair<int, int> pos) {
    return encodePos(pos.first, pos.second);
}

std::tuple<std::vector<const Cell*>, std::vector<const Cell*>,
           std::vector<const Cell*>>
boxIntersection(int boxIndex, int lineType, int lineIndex, const Grid& grid) {
    if (lineType) {
        if (lineIndex / 3 != boxIndex % 3)
            return std::make_tuple(std::vector<const Cell*>(),
                                   std::vector<const Cell*>(),
                                   std::vector<const Cell*>());
    } else {
        if (lineIndex / 3 != boxIndex / 3)
            return std::make_tuple(std::vector<const Cell*>(),
                                   std::vector<const Cell*>(),
                                   std::vector<const Cell*>());
    }
    std::vector<const Cell*> intersect;
    std::vector<const Cell*> boxRemaining;
    std::vector<const Cell*> lineRemaining;

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

bool targetIn(int target, std::vector<const Cell*> cells) {
    for (auto cell : cells) {
        if (cell->candidates[target]) return true;
    }
    return false;
}

uint8_t encodePos(int x, int y) { return x << 4 | y; }

bool sees(const Cell* cell1, const Cell* cell2) {
    return sees(cell1->x, cell1->y, cell2->x, cell2->y);
}

bool cmp(const Cell* cell1, const Cell* cell2){
    if(cell1->x < cell2->x) return true;
    if(cell1->x== cell2->x && cell1->y < cell2->y) return true;
    return false;
}