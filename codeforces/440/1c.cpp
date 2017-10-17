#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

int coord_comp(VL &x) {
  set<ll> s;
  REP(i, 0, x.size()) {
    s.insert(x[i]);
  }
  vector<ll> v(s.begin(), s.end());
  sort(v.begin(), v.end());
  map<ll, int> tbl;
  REP(i, 0, v.size()) {
    tbl[v[i]] = i;
  }
  REP(i, 0, x.size()) {
    x[i] = tbl[x[i]];
  }
  return tbl.size();
}

const int N = 200100;
VI edges[N];
bool vis[N];

PI dfs(int v) {
  if (vis[v]) {
    return PI(0, 0);
  }
  vis[v] = true;
  int nv = 1;
  int ne = edges[v].size();
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    PI res = dfs(w);
    nv += res.first;
    ne += res.second;
  }
  return PI(nv, ne);
}

ll pow2[N];
void init(void) {
  pow2[0] = 1;
  REP(i, 1, N) {
    pow2[i] = pow2[i - 1] * 2 % mod;
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n;
  cin >> n;
  VL x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  int xs = coord_comp(x);
  int ys = coord_comp(y);
  REP(i, 0, n) {
    edges[x[i]].push_back(xs + y[i]);
    edges[xs + y[i]].push_back(x[i]);
  }
  ll prod = 1;
  REP(i, 0, xs + ys) {
    if (vis[i]) { continue; }
    PI tmp = dfs(i);
    int nv = tmp.first;
    assert (tmp.second % 2 == 0);
    int ne = tmp.second / 2;
    assert (ne >= nv - 1);
    ll mul;
    if (ne == nv - 1) { // This connected component is a tree
      mul = (pow2[nv] + mod - 1) % mod;
    } else {
      mul = pow2[nv];
    }
    prod = prod * mul % mod;
  }
  cout << prod << "\n";
}
