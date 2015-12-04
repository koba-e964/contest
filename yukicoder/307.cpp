#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;


VI a[201];
int h,w;

int get_col(int x, int y) {
    if (x < 0 || x >= h || y < 0 || y >= w) {
        return -1;
    }
    return a[x][y];
}

int main(void){
    cin >> h >> w;
    REP(i, 0, h) {
        a[i].reserve(w);
        REP(j, 0, w) {
            int b;
            cin >> b;
            a[i].push_back(b);
        }
    }
    int q;
    int saturated = 0;
    int sat_col = -1;
    cin >> q;
    REP(loop, 0, q) {
        int r,c,x;
        cin >> r >> c >> x;
        r--,c--;
        if (saturated) {
            sat_col = x;
            continue;
        }
        int col = a[r][c];
        if (x == col) {
            continue;
        }
        int dir[5] = {0,1,0,-1,0};
        queue<PI> que;
        que.push(PI(r, c));
        int cnt = 0;
        while (!que.empty()) {
            PI t = que.front(); que.pop();
            int x = t.first, y = t.second;
            if (a[x][y] == 1 - col) {
                continue;
            }
            REP(d, 0, 4) {
                int dx = dir[d], dy = dir[d + 1];
                if (get_col(x + dx, y + dy) == col) {
                    que.push(PI(x + dx, y + dy));
                }
            }
            a[x][y] = 1 - col;
            cnt++;
        }
        if (cnt >= w * h) {
            saturated = 1;
            sat_col = a[r][c];
        }
    }
    REP(i, 0, h) {
        REP(j, 0, w) {
            if (saturated) {
                a[i][j] = sat_col;
            }
            cout << a[i][j] << (j == w - 1 ? "\n" : " ");
        }
    }
}
