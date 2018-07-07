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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;
const ll mod = 1e9 + 7;

const int N = 100100;

vector<PIL> g[N];

ll sgn[N], seg[N];
VL poss;
bool vis[N];
bool mess;
// x[v] = a * x[0] + b
void dfs(int v, ll a, ll b) {
  if (DEBUG) {
    DEBUGP(v);
    DEBUGP(a);
    DEBUGP(b);
  }
  if (vis[v]) {
    if (a == sgn[v]) {
      if (b != seg[v]) mess = 1;
      return;
    }
    if ((b - seg[v]) % (a - sgn[v]) != 0) {
      mess = 1;
    } else {
      if (DEBUG) {
        cerr << "v=" << v << " " << a<<"x+"<<b<<"="<<sgn[v]<<"x+"<<seg[v]<<endl;
      }
      poss.push_back(-(b - seg[v]) / (a - sgn[v]));
    }
    return;
  }
  sgn[v] = a;
  seg[v] = b;
  vis[v] = 1;
  for (PIL ws: g[v]) {
    int w = ws.first;
    ll s = ws.second;
    dfs(w, -a, s - b);
  }
}

const ll inf = 1e18;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int u, v;
    ll s;
    cin >> u >> v >> s;
    u--, v--;
    g[u].push_back(PIL(v, s));
    g[v].push_back(PIL(u, s));
  }
  dfs(0, 1, 0);
  if (mess) {
    cout << 0 << endl;
    return 0;
  }
  sort(poss.begin(), poss.end());
  poss.erase(unique(poss.begin(), poss.end()), poss.end());
  if (DEBUG) {
    for (int p:poss) cerr <<" " << p;
    cerr << endl;
  }
  if (poss.size() >= 1) {
    if (poss.size() >= 2) {
      cout << 0 << endl;
      return 0;
    }
    ll x = poss[0];
    bool ok = true;
    REP(i, 0, n) {
      if (sgn[i] * x + seg[i] <= 0) {
        ok = false;
      }
    }
    cout << (ok ? 1 : 0) << endl;
    return 0;
  }
  ll lo = 1, hi = inf;
  REP(i, 0, n) {
    if (sgn[i] == 1) {
      lo = max(lo, -seg[i] + 1);
    } else {
      hi = min(hi, seg[i] - 1);
    }
  }
  DEBUGP(lo);
  DEBUGP(hi);
  cout << (hi >= lo ? hi - lo + 1 : 0) << endl;
}
