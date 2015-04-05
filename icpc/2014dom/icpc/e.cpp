#include <iostream>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

#define rep(i, n)       reps(i, 0, n)
#define reps(i, m, n)   for (int i = m; i < int(n); ++i)

const int INF = 1001001001;
typedef pair<bool, int> P;

struct Edge { int to, cost; };
vector<Edge> G[811];
bool mark[811];
int s, t;

bool check(int v, int par) {
    if (v == t) {
        mark[v] = true;
        return true;
    }
    rep (i, G[v].size()) {
        if (G[v][i].to == par) continue;
        if (check(G[v][i].to, v)) {
            mark[v] = true;
            return true;
        }
    }
    return false;
}

int dfs(int v, int par) {
    int ret = 0;
    bool ch = false;
    rep (i, G[v].size()) {
        if (G[v][i].to == par) {
            ret += G[v][i].cost;
            if (!mark[v]) ret += G[v][i].cost;
            continue;
        }
        ch = true;
        ret += dfs(G[v][i].to, v);
    }
    return ch ? ret : 0;
}

int main()
{
    int n;
    while (cin >> n && n) {
        int sc = 0;
        vector<int> p(n), d(n);
        rep (i, n) G[i].clear();
        rep (i, n - 1) cin >> p[i];
        rep (i, n - 1) cin >> d[i];
        rep (i, n - 1) {
            Edge e1, e2;
            e1.to = p[i] - 1;
            e2.to = i + 1;
            e1.cost = e2.cost = d[i];
            G[i + 1].push_back(e1);
            G[p[i] - 1].push_back(e2);
            sc += d[i];
        }
        int ans = INF;
        rep (i, n) rep (j, n) {
            s = i;
            t = j;
            fill(mark, mark + n, false);
            check(s, -1);
            ans = min(ans, dfs(s, -1));
        }
        cout << sc + ans << endl;
    }
}

