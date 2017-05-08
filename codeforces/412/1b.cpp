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
const ll mod = 1e9 + 7;
const ll inf = mod + 1;

ll check(const vector<VI> &a, VI path, ll &mi_gl) {
  // Is participant 1 winning?
  int n = a.size();
  int p1 = 0;
  int p2 = 0;
  vector<VI> sc(2, VI(5, 0));
  REP(i, 0, 5) {
    if (a[0][i] >= 0) {
      p1 += (sc[0][i] = path[i] / 250 * (250 - a[0][i]));
    }
    if (a[1][i] >= 0) {
      p2 += (sc[1][i] = path[i] / 250 * (250 - a[1][i]));
    }
  }
  if (p1 <= p2) {
    return inf;
  }
  ll hi = inf;
  VI cor(5, 0);
  REP(i, 0, 5) {
    REP(j, 0, n) {
      if (a[j][i] >= 0) {
	cor[i] += 1;
      }
    }
  }
  for (ll mid = min(mi_gl, 120LL * 100); mid >= 0; --mid) {
    bool ok = true;
    ll sub_granted = 0;
    REP(i, 0, 5) {
      int ratio = 1 << (path[i] / 500 - 1);
      int sn = ratio == 32 ? -1 : 1;
      int sd = ratio == 32 ? 1 : ratio * 2;
      int tn = 1;
      int td = ratio;
      int x = cor[i];
      ll ma = mid;
      ll mi = 0;
      ma = min(ma, tn * (n + mid) / td - x);
      mi = max(mi, sn * (n + mid) / sd - x + 1);
      if (a[0][i] == -1) {
        ma = min(ma, 0LL);
      }
      if (mi > ma) {
	ok = false;
	break;
      }
      sub_granted += ma;
      if (0) {
	cerr << "mid = " << mid << ", [" << i << "] ";
	cerr << "[" << mi << ", " << ma << "]" << endl;
      }
    }
    if (ok) {
      hi = mid;
    }
  }
  if (0) {
    cerr << "path:";
    REP(i, 0, 5) {
      cerr << " " << path[i];
    }
    cerr << endl << "1:";
    REP(i, 0, 5) {
      cerr << " " << sc[0][i];
    }
    cerr << endl << "2:";
    REP(i, 0, 5) {
      cerr << " " << sc[1][i];
    }
    cerr << endl << "cnt = " << hi << endl;
  }
  return hi;
}

void dfs(const vector<VI> &a, int v, VI path, ll &mi) {
  if (v >= 5) {
    ll res = check(a, path, mi);
    mi = min(mi, res);
    return;
  }
  REP(i, 0, 6) {
    VI t(path);
    t.push_back(500 * (i + 1));
    dfs(a, v + 1, t, mi);
  }
}

int main(void){
  int n;
  cin >> n;
  vector<VI> a(n, VI(5));
  REP(i, 0, n) {
    REP(j, 0, 5) {
      cin >> a[i][j];
    }
  }
  ll mi = inf;
  dfs(a, 0, VI(), mi);
  cout << (mi == inf ? -1 : mi) << endl;
}
