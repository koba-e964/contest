#include <iostream>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

#define rep(i, n)       reps(i, 0, n)
#define reps(i, m, n)   for (int i = m; i < int(n); ++i)

int h;
int cell[10][5];

int score;
#define DEBUG 0

void fall(void) {
    rep(i, 5) {
        vector<int> ls;
        rep(j,h) {
            if(cell[j][i])
                ls.push_back(cell[j][i]);
            cell[j][i]=0;
        }
        int s = ls.size();
        reps(j,h-s,h) {
            cell[j][i] = ls[j+s-h];
        }
    }
}

bool next(void) {
    bool mod=false;
    rep(i,h) {
        bool mi = false;
        rep(j,3) {
            if(mi)break;
            int c = cell[i][j];
            if(c==0)continue;
            int t=0;
            for(; t<5-j;++t) {
                if(cell[i][j+t] != c) {
                    break;
                }
            }
            if(t>=3) { //delete
                mi = true;
                score += t * c;
                rep(k,t) {
                    cell[i][j+k] = 0;
                }
            }
        }
        if(mi) mod=true;
    }
    return mod;
}

int main()
{
    while(1) {
        cin>>h;
        if(h==0)break;
        rep(i,h) {
             rep(j,5) {
                 cin >> cell[i][j];
             }
        }
        score = 0;
        while(next()){
            if(DEBUG) {
                rep(i,h) {
                    rep(j,5) {
                        cout << cell[i][j] << " ";
                    }
                }
                cout << endl;
            }
            fall();}
        cout << score << endl;
    }
}

