#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;
typedef pair<int, int> PI;

const int N = 123456;
VI edges[N];
int n;

VI bfs(int v, bool leaf) {
    vector<bool> vis(n);
    VI ret;
    if (not leaf) {
        ret = VI(n);
    }
    queue<PI> que;
    que.push(PI(0, v));
    while (not que.empty()) {
        PI t = que.front(); que.pop();
        int d = t.first;
        int v = t.second;
        if (vis[v]) {
            continue;
        }
        vis[v] = true;
        if (not leaf) {
            ret[v] = d;
        } else {
            if (edges[v].size() == 1) {
                ret.push_back(d);
            }
        }
        REP(i, 0, edges[v].size()) {
            int w = edges[v][i];
            if (vis[w]) { continue; }
            que.push(PI(d + 1, w));
        }
    }
    return ret;
}

int suprem(VI &vec) {
    if (vec.size() <= 1) {
        return 0;
    }
    sort(vec.rbegin(), vec.rend());
    return vec[0] - vec[1];
}

int main(void) {
    cin >> n;
    REP(_, 0, n - 1) {
        int u, v;
        cin >> u >> v;
        u--, v--;
        edges[u].push_back(v);
        edges[v].push_back(u);
    }
    VI d0 = bfs(0, false);
    PI ma(-1, -1);
    REP(i, 0, n) {
        ma = max(ma, PI(d0[i], i));
    }
    int opp = ma.second;
    VI dopp = bfs(opp, false);
    ma = PI(-1, -1);
    REP(i, 0, n) {
        ma = max(ma, PI(dopp[i], i));
    }
    int opp2 = ma.second;
    VI dopp2l = bfs(opp2, true);
    VI doppl = bfs(opp, true);
    if (doppl.size() == 2) {
        cout << n - 1 << endl;
    } else {
        cout << suprem(doppl) + suprem(dopp2l) << endl;
    }
}
