#include "lockedCandidates.h"

#include "util.h"

std::pair<bool, std::string> findLockedCandidates(const Grid &grid) {
    for (int lineType : {0, 1}) {
        for (int box = 0; box < 9; box++) {
            for (int line = 0; line < 9; line++) {
                auto res = boxIntersection(box, lineType, line, grid);
                auto &intersection = std::get<0>(res);
                auto &boxRemaining = std::get<1>(res);
                auto &lineRemaining = std::get<2>(res);
                for (int target = 0; target < 9; target++) {
                    if(!targetIn(target, intersection))continue;
                    if(!targetIn(target, boxRemaining) && targetIn(target,lineRemaining)){
                        std::string prompt = "Locked Candidates Type 1 (Pointing): ";
                        std::vector<std::reference_wrapper<const Cell>> toEli;
                        for(auto &c : lineRemaining){
                            if(c.get().candidates[target]) toEli.push_back(c);
                        }
                        prompt += formater(toEli);   
                        prompt += "!=";
                        prompt += char(target + '1');
                        return std::make_pair(true, prompt);                   
                    }
                    if(!targetIn( target, lineRemaining) && targetIn(target, boxRemaining)){
                        std::string prompt = "Locked Candidates Type 2 (Claiming): ";
                        std::vector<std::vector<std::reference_wrapper<const Cell>>> toEli;
                        for(auto c : boxRemaining){
                            if(c.get().candidates[target]){
                                toEli[c.get().x % 3].push_back(c);
                            }
                        }
                        prompt += formater(toEli);
                        prompt += "!=";
                        prompt += char(target + '1');
                        return std::make_pair(true, prompt);   
                        
                    }
                }
            }
        }
    }
    return std::make_pair(false, "");
}