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
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

const int N = 100100;
int n;
vector<PIL> g[N];
vector<PL> edges;

VI centroid;

ll evenly;

// Returns the size of the subtree whose root is v.
int dfs(int v, int p) {
  int sz = 1;
  bool is_centroid = 1;
  for (PIL wc: g[v]) {
    int w = wc.first;
    ll c = wc.second;
    if (w == p) continue;
    int sub = dfs(w, v);
    sz += sub;
    if (2 * sub >= n) {
      is_centroid = 0;
    }
    if (2 * sub == n) {
      evenly = c;
    }
    int opp = n - sub;
    int times = opp == sub ? 2 * sub - 1 : 2 * min(opp, sub);
    edges.push_back(PL(c, times));
  }
  if (2 * sz < n) {
    is_centroid = 0;
  }
  if (is_centroid) centroid.push_back(v);
  return sz;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    a--, b--;
    g[a].push_back(PIL(b, c));
    g[b].push_back(PIL(a, c));
  }
  dfs(0, -1);
  sort(edges.begin(), edges.end());
  ll tot = 0;
  REP(i, 0, edges.size()) {
    PL e = edges[i];
    tot += e.first * e.second;
  }
  if (evenly == 0) {
    ll mi = 1e17;
    assert (centroid.size() == 1);
    int v = centroid[0];
    REP(i, 0, g[v].size()) {
      mi = min(mi, g[v][i].second);
    }
    tot -= mi;
  }
  cout << tot << endl;
}
