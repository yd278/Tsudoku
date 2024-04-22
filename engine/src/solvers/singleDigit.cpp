#include "solvers/singleDigit.h"

#include <utility>

#include "util.h"

void findTurbotFish(Grid &grid) {
    grid.initInsAndExe();
    auto allStrongLinks = grid.getStrongLinks();
    FOR_ALL(target) {
        // iterate two strong links
        auto &strongLinks = (*allStrongLinks)[target];
        int n = strongLinks.size();
        for (int i = 0; i < n; i++) {
            auto link1 = std::vector<const Cell *>{strongLinks[i].first,
                                                   strongLinks[i].second};

            for (int j = i + 1; j < n; j++) {
                auto link2 = std::vector<const Cell *>{strongLinks[j].first,
                                                       strongLinks[j].second};
                // check if either sees each other
                for (int b1 : {0, 1}) {
                    for (int b2 : {0, 1}) {
                        if (sees(link1[b1], link2[b2]) && link1[b1]!=link2[b2]) {
                            std::vector<const Cell *> bridge = {link1[b1],
                                                                link2[b2]};
                            std::vector<const Cell *> pincer = {link1[1 - b1],
                                                                link2[1 - b2]};

                            // two pincers should not see each other
                            if (sees(pincer[0], pincer[1])) continue;
                            // turbot fish pattern find;

                            // determine order
                            if (cmp(bridge[1], bridge[0])) {
                                std::swap(bridge[0], bridge[1]);
                                std::swap(pincer[0], pincer[1]);
                            }

                            // determine type:
                            int type = 0;  // 0 SC, 1 Kite, 2 TF
                            if (link1[0]->x != link1[1]->x &&
                                link1[0]->y !=
                                    link1[1]
                                        ->y) {  // strong link in the same box
                                type = 2;
                            } else if (link2[0]->x != link2[1]->x &&
                                       link2[0]->y != link2[1]->y) {
                                type = 2;
                            } else {
                                int houseType1 =
                                    link1[0]->y ==
                                    link1[1]->y;  // 1 :column; 0 :row
                                int houseType2 = link2[0]->y == link2[1]->y;
                                type = houseType1 != houseType2;
                            }

                            // set insts;
                            grid.initInsAndExe();
                            grid.setExec(false);

                            grid.addInst(0x50 + type);

                            for (int index : {0, 1})
                                grid.addInst(encodePos(bridge[index]));
                            for (int index : {0, 1})
                                grid.addInst(encodePos(pincer[index]));
                            grid.addInst(target);

                            // set exec
                            bool flag = false;
                            FOR_ALL(ei) FOR_ALL(ej) {
                                if (sees(pincer[0], ei, ej) &&
                                    sees(pincer[1], ei, ej)) {
                                    if (grid.getCell(ei, ej)
                                            ->candidates[target]) {
                                        grid.addExec(encodePos(ei, ej), target);
                                        flag = true;
                                    }
                                }
                            }
                            if (flag) {
                                grid.addExecToInst();
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
void findEmptyRectangle(Grid &grid) {}