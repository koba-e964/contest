#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;
VI g[N];

int dfs(int v, int par) {
  REP(i, 0, g[v].size()) {
    int w = g[v][i];
    if (par == w) continue;
    return dfs(w, v);
  }
  return v;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  VI tri;
  REP(i, 0, n) {
    if (g[i].size() >= 3) {
      tri.push_back(i);
    }
  }
  if (tri.size() >= 2) {
    cout << "No\n";
    return 0;
  }
  int root = -1;
  if (tri.size() == 1) {
    root = tri[0];
  } else {
    root = 0;
  }
  cout << "Yes\n";
  cout << g[root].size() << "\n";
  REP(i, 0, g[root].size()) {
    int w = g[root][i];
    int leaf = dfs(w, root);
    cout << root + 1 << " " << leaf + 1 << "\n";
  }
}
