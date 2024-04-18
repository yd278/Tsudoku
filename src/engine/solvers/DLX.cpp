#include <iostream>
#include <stack>
#include <vector>
class DLX {
    int n, m;
    int tail;
    std::vector<int> first, size;
    std::vector<int> L, R, U, D;
    std::vector<int> col, row;
    std::stack<int> stk;
   public:

    std::vector<std::vector<int>> ans;
    bool flag;
    DLX(int r, int c) {
        flag = false;
        int num = (r+1) * c+1;
        first.resize(r+1);
        size.resize(c);
        ans.resize(10);
        for(auto &row : ans){
            row.resize(10);
        } 
        L.resize(num);
        R.resize(num);
        U.resize(num);
        D.resize(num);
        col.resize(num);
        row.resize(num);
        //build the first row 
        for(int i = 0; i <= c; i++){
            L[i] = i-1;
            R[i] = i+1;
            U[i] = D[i] = i;
        }
        L[0] = c;
        R[c] = 0;
        tail = c;
    }

    void insert(int r, int c){
        tail++;
        col[tail] = c;
        row[tail] = r;
        size[c]++;
        
        D[tail] = D[c];
        U[D[c]]= tail;
        U[tail] = c;
        D[c] = tail;

        if(!first[r]){
            first[r] = L[tail] = R[tail] = tail; 
        }else{
            R[tail] = R[first[r]];
            L[R[tail]] = tail;

            L[tail] = first[r];
            R[first[r]]=tail;

        }
    }

    void remove(int pos){
        int i,j;
        L[R[pos]] = L[pos];
        R[L[pos]] = R[pos];
        for(i = D[pos]; i != pos; i= D[i]){
            for(j = R[i]; j!=i; j = R[j]){
                U[D[j]] = U[j];
                D[U[j]] = D[j];
                size[col[j]]--;
            }
        }
    }

    void recover(int pos){
        int i,j;
        for(i = U[pos];i!=pos; i= U[i]){
            for(j = L[i]; j!=i;j=L[j]){
                U[D[j]] = D[U[j]]=j;
                size[col[j]]++;
            }
        }
        L[R[pos]] = R[L[pos]] = pos;
    }

    void dance(int dep){
        int i,j,cur = R[0];
        if(!R[0]){
            if(flag) throw std::invalid_argument("multiple solutions");
            flag = true;
            std::stack<int> tmp;
            while(!stk.empty()){
                int t = stk.top();
                stk.pop();
                int x = (t-1)/9/9+1;
                int y = (t-1)/9%9+1;
                int v = (t-1)%9+1;
                ans[x][y] = v;
                tmp.push(t);
            }
            while(!tmp.empty()){
                stk.push(tmp.top());
                tmp.pop();
            }
        }
        for(i = R[0]; i!=0; i=R[i]){
            if(size[i] < size[cur]) cur = i;
        }
        remove(cur);
        for(i = D[cur]; i!=cur; i=D[i]){
            stk.push(row[i]);
            for(j = R[i]; j!=i; j=R[j]) remove(col[j]);
            dance(dep+1);
            for(j = L[i];j!=i;j=L[j]) recover(col[j]);
            stk.pop();
        }
        recover(cur);
    }
};

int getId(int row, int col, int num){
    return (row - 1) * 9 * 9 + (col - 1) * 9 + num;
}

void insert(int row,int col,int num, DLX &dlx){
  int dx = (row - 1) / 3 + 1;
  int dy = (col - 1) / 3 + 1;
  int room = (dx - 1) * 3 + dy;
  int id = getId(row, col, num);
  int f1 = (row - 1) * 9 + num;            // task 1
  int f2 = 81 + (col - 1) * 9 + num;       // task 2
  int f3 = 81 * 2 + (room - 1) * 9 + num;  // task 3
  int f4 = 81 * 3 + (row - 1) * 9 + col;   // task 4
  dlx.insert(id,f1);
  dlx.insert(id,f2);
  dlx.insert(id,f3);
  dlx.insert(id,f4);
}

void init(DLX &dlx, std::string &puzzle){
    for(int i = 0; i < 81; i++){
        int x = i / 9 + 1;
        int y = i % 9 + 1;
        int v = puzzle[i] - '0';
        dlx.ans[x][y] = v;
        for(int k = 1; k<= 9; k++){
            if(v && v!=k) continue;
            insert(x, y, k, dlx);
        }
    }
}

std::string format(DLX &dlx){
    std::string res = "";
    for(int i = 1; i <= 9; i++){
        for(int j = 1; j <= 9; j++){
            res += dlx.ans[i][j] + '0';
        }
    }
    return res;
}

std::string solve(std::string puzzle){
    DLX dlx(729,324);
    init(dlx,puzzle);
    try {
    dlx.dance(0);
    } catch (std::invalid_argument &e) {
        throw;
    }
    if(dlx.flag) return format(dlx);
    else throw std::invalid_argument("No Solution");
}

