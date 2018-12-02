#include <bits/stdc++.h>

using namespace std;

#define rep(i, n) for (int i = 0; i < int(n); ++i)

const int N = 516000;
typedef vector<int> VI;
VI edges[N];

int dist[N];
int path_pres[N];

void dfs(int v, int par = -1, int d = 0) {
    dist[v] = d;
    rep(i, edges[v].size()) {
        int w = edges[v][i];
        if (w == par) { continue; }
        dfs(w, v, d + 1);
    }
}
void dfs2(int v, int par, set<int> &vis, int d = 0) {
    dist[v] = d;
    vis.insert(v);
    rep(i, edges[v].size()) {
        int w = edges[v][i];
        if (w == par) { continue; }
        dfs2(w, v, vis, d + 1);
    }
}
void dfs3(int v, const set<int> &vis, int par = -1, int d = 0) {
    dist[v] = d;
    rep(i, edges[v].size()) {
        int w = edges[v][i];
        if (w == par) { continue; }
        if (vis.count(w) == 0) { continue; }
        dfs3(w, vis, v, d + 1);
    }
}

int subtree_diam(int v) {
    int ma = 0;
    rep(i, edges[v].size()) {
        int w = edges[v][i];
        if (path_pres[w]) { continue; }
        set<int> vis;
        dfs2(w, v, vis);
        int maxi = w;
        for (set<int>::iterator it = vis.begin(); it != vis.end(); ++it) {
            if (dist[maxi] < dist[*it]) { maxi = *it; }
        }
        dfs3(maxi, vis);
        int maxi2 = w;
        for (set<int>::iterator it = vis.begin(); it != vis.end(); ++it) {
            if (dist[maxi2] < dist[*it]) { maxi2 = *it; }
        }
        ma = max(ma, dist[maxi2] + 1);
    }
    return ma;
}

int main(){
    // Read the number of nodes
    int n;
    cin >> n;
    for(int a0 = 0; a0 < n-1; a0++){
        // Read each edge and build tree
        int u;
        int v;
        cin >> u >> v;
        u--, v--;
        edges[u].push_back(v);
        edges[v].push_back(u);
    }
    dfs(0);
    int maxi = 0;
    rep(i, n) {
        if (dist[maxi] < dist[i]) { maxi = i; }
    }
    dfs(maxi);
    int maxi2 = 0;
    rep(i, n) {
        if (dist[maxi2] < dist[i]) { maxi2 = i; }
    }
    VI path;
    int cur = maxi2;
    while (dist[cur] > 0) {
        path.push_back(cur);
        rep(i, edges[cur].size()) {
            int w = edges[cur][i];
            if (dist[w] < dist[cur]) {
                cur = w;
                break;
            }
        }
    }
    path.push_back(maxi);
#if 0
    cerr << "diam: ";
    rep(i, path.size()) {
        cerr << path[i] << (i == path.size() - 1 ? "\n" : "====");
    }
#endif
    int pathlen = path.size() - 1;
    rep(i, path.size()) { path_pres[path[i]] = 1; }
    int ma = 0;
    rep(i, path.size()) {
        ma = max(ma, subtree_diam(path[i]));
    }
    cout << pathlen + ma << endl;
    //  Print the maximum possible diameter you can achieve by splitting the tree once and reconnecting it
    return 0;
}
