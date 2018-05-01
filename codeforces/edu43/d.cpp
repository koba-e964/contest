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

// https://www.sciencedirect.com/science/article/pii/S0166218X06003854
VI restore_deg_seq(int nv, const VI &d) {
  int n = d.size();
  if (n <= 0) return VI(nv, 0);
  if (n == 1) {
    assert (nv >= d[0] + 1);
    VI ret(nv);
    REP(i, 0, d[0] + 1) ret[i] = d[0];
    sort(ret.begin(), ret.end());
    return ret;
  }
  int suc = d[n - 1] + 1;
  VI sub(n - 1);
  REP(i, 0, n - 1) sub[i] = d[n - 1] - d[i];
  sort(sub.begin(), sub.end());
  VI ret(nv, 0);
  REP(i, 0, suc) ret[i] = d[n - 1];
  VI subreg = restore_deg_seq(suc - 1, sub);
  REP(i, 1, suc) ret[i] -= subreg[i - 1];
  sort(ret.begin(), ret.end());
  if (0) {
    cerr << "rds(" << nv << ",";
    REP(i, 0, n) cerr << " " << d[i];
    cerr << ")=";
    REP(i, 0, nv) cerr<<" " << ret[i];
    cerr << endl;
  }
  return ret;
}

#define MOCK 0

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI d(n);
#if MOCK
  REP(i, 0, n) d[i] = 1000 - (n - i);
#else
  REP(i, 0, n) cin >> d[i];
#endif
  int nv = d[n - 1] + 1;
  VI deg = restore_deg_seq(nv, d);
  if (0) {
    cerr << "deg:";
    REP(i, 0, deg.size()) {
      cerr << " " << deg[i];
    }
    cerr << endl;
  }
  vector<PI> pool;
  REP(i, 0, nv) {
    pool.push_back(PI(deg[i], i));
  }
  vector<PI> edges;
  while (pool.size() > 0) {
    sort(pool.begin(), pool.end());
    PI b = pool.back(); pool.pop_back();
    int db = b.first;
    int sz = pool.size();
    REP(u, sz - db, sz) {
      edges.push_back(PI(pool[u].second, b.second));
      pool[u].first--;
    }
    vector<PI> nxt;
    REP(i, 0, sz) {
      if (pool[i].first > 0) nxt.push_back(pool[i]);
    }
    pool = nxt;
  }
  cout << edges.size() << "\n";
#if !MOCK
  REP(i, 0, edges.size()) {
    cout << edges[i].first + 1 << " " << edges[i].second + 1 << "\n";
  }
#endif
}
