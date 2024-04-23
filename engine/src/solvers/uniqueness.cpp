#include "solvers/uniqueness.h"

#include "util.h"

void uniquenessTestType1(Grid &grid) {
    // no more than two bivalues with same candidates in one house:
    // as we always execute naked pair before uniqueness test, these two
    // bivalues are strong linked
    
    auto bivaluesMap = grid.getBiValuesByCands();
    FOR_ALL(x) for (int y = 0; y < x; y++) {
        auto bivalues = (*bivaluesMap)[x][y];
        for(auto cell : bivalues){

            if(!cell->SL[x]) continue;
            if(!cell->SL[y]) continue;
            if(cell->SL[x] != cell->SL[y])continue;

            if(!cell->SL[x+9]) continue;
            if(!cell->SL[y+9]) continue;
            if(cell->SL[x+9] != cell->SL[y+9])continue;

            // type1 pattern found:
            auto pincer1 = cell->SL[x];
            auto pincer2 = cell->SL[x+9];
            auto exec = grid.getCell(pincer2->x,pincer1->y);
            if((exec->candidates&cell->candidates).count()>0){
                grid.initInsAndExe();
                grid.addInst(0x60);
                int row1 = cell->x;
                int row2 = pincer2->x;
                if(row1 > row2) std::swap(row1,row2);
                grid.addInst((row1<<4)|0xF);
                grid.addInst((row2<<4)|0xF);


                int col1 = cell->y;
                int col2 = pincer1->y;
                if(col1 > col2) std::swap(col1,col2);
                grid.addInst(0xF0|col1);
                grid.addInst(0xf0|col2);

                grid.addInst(y);
                grid.addInst(x);
                if(exec->candidates[y])
                    grid.addExec(encodePos(exec),y);
                if(exec->candidates[x]){
                    grid.addExec(encodePos(exec),x);
                }

                grid.addExecToInst();
                return;
            }



        }
    }
}
void uniquenessTestType2(Grid &grid) {}
void uniquenessTestType3(Grid &grid) {}
void uniquenessTestType4(Grid &grid) {}
void uniquenessTestType5(Grid &grid) {}
void findHiddenRectangle(Grid &grid) {}
void avoidableRectangle1(Grid &grid) {}
void avoidableRectangle2(Grid &grid) {}

void bivalueUniversalGraveP1(Grid &grid) {
    // check  all bi-values
    const Cell *tri = nullptr;
    FOR_ALL(i) FOR_ALL(j) {
        auto cell = grid.getCell(i, j);
        if (cell->value) continue;
        if (cell->candidates.count() == 3) {
            if (tri == nullptr)
                tri = cell;
            else
                return;

        } else if (cell->candidates.count() != 2)
            return;
    }
    if (tri == nullptr) return;

    // check extra candidate
    int extra = -1;

    FOR_ALL(can) {
        if (!tri->candidates[can]) continue;
        // test row:
        int cnt = 0;
        FOR_ALL(col) {
            if (grid.getCell(tri->x, col)->candidates[can]) cnt++;
        }
        if (cnt == 2) continue;  // either not BUG, or not extra
        if (cnt == 3) {
            if (extra == -1)
                extra = can;
            else
                return;  // not BUG
        }
        if (cnt != 3) return;  // not BUG
        cnt = 0;
        FOR_ALL(row) {
            if (grid.getCell(row, tri->y)->candidates[can]) cnt++;
        }
        if (cnt != 3) return;

        int box = (tri->x / 3) * 3 + (tri->y / 3);
        cnt = 0;
        FOR_ALL(cell) {
            if (grid.getCell(2, box, cell)->candidates[can]) cnt++;
        }
        if (cnt != 3) return;
    }
    if (extra == -1) return;

    // check bug

    for (int houseType : {0, 1, 2}) {
        FOR_ALL(houseId) {
            std::vector<int> count(9);
            FOR_ALL(cellId) {
                auto cell = grid.getCell(houseType, houseId, cellId);

                if (cell->value) continue;
                FOR_ALL(can) {
                    if (cell->candidates[can]) count[can]++;
                }
                if (cell == tri) count[extra]--;
            }
            FOR_ALL(can) {
                if (count[can] != 0 && count[can] != 2) return;
            }
        }
    }

    // bug find
    grid.initInsAndExe();
    grid.setExec(true);
    grid.addInst(0x69);
    grid.addExec(encodePos(tri), extra);
    grid.addExecToInst();
}