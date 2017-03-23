#include <iostream>
#include <algorithm>
#include <vector>
#include <cstdio>
using namespace std;
 
const int N = 123456;
typedef vector<int> VI;
typedef pair<int, int> PI;
vector<PI> edges[N];
 
#define rep(i, n) for (int i = 0; i < int(n); ++i)
 
double p;
 
 
double dfs(int v, int par) {
    double sum = 0;
    rep(i, edges[v].size()) {
        PI w = edges[v][i];
        if (par == w.first) continue;
        double res = dfs(w.first, v) + w.second;
        sum += res * p;
    }
    return sum;
}
 
double dfs2(int v, int par, double single) {
    double sum = single;
    rep(i, edges[v].size()) {
        PI w = edges[v][i];
        if (par == w.first) continue;
        double res = dfs2(w.first, v, single) + w.second;
        sum += res * p;
    }
    return sum;
}
 
 
// RUPC C
int main() {
    int n;
    cin >> p >> n;
    rep(i, n - 1) {
        int x, y, c;
        cin >> x >> y >> c;
        x--, y--;
        edges[x].push_back(PI(y, c));
        edges[y].push_back(PI(x, c));
    }
    double single = dfs(0, -1); // T
    double ret = dfs2(0, -1, single);
    printf("%.15f\n", ret);
    // your code goes here
    return 0;
}
