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

std::string formater(int x, int y) {
    std::string s = "r";
    s += (x + '1');
    s += "c";
    s += (y + '1');
    return s;
}

std::string formater(const Cell &c) { return formater(c.x, c.y); }

std::string formater(std::vector<std::reference_wrapper<const Cell>> line) {
    if (line.size() == 1) return formater(line[0]);
    std::string s = "r";
    if (line[0].get().x == line[1].get().x) {
        s += (line[0].get().x + '1');
        std::bitset<9> tmp;
        for (auto c : line) {
            tmp.set(c.get().y);
        }

        s += "c";
        for (int i = 0; i < 9; i++) {
            if (tmp[i]) s += char(i + '1');
        }
    } else {
        std::bitset<9> tmp;
        for (auto c : line) {
            tmp.set(c.get().x);
        }
        for (int i = 0; i < 9; i++) {
            if (tmp[i]) s += char(i + '1');
        }
        s+="c";
        s+=char(line[9].get().y + '1');
    }
    return s;
}


std::string formater(std::vector<std::vector<std::reference_wrapper<const Cell>>> lines){
    std::string res = "";
    for(auto &line : lines){
        if(!line.empty()) res += formater(line) + ",";
    }
    if(res.length()>1)res.pop_back();
    return res;
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

    int boxSubIndex = lineType ? boxIndex % 3 : boxIndex / 3;
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