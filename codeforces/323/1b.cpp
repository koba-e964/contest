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

vector<VI> mul(const vector<VI> &a, const vector<VI> &b) {
  int n = a.size();
  vector<VI> ret(n, VI(n, 0));
  REP(i, 0, n) {
    REP(j, 0, n) {
      int r = -1; // -1 for -inf
      REP(k, 0, n) {
	if (a[i][k] >= 0 && b[k][j] >= 0) {
	  r = max(r, a[i][k] + b[k][j]);
	}
      }
      ret[i][j] = r;
    }
  }
  return ret;
}

int main(void){
  int n;
  ll t;
  cin >> n >> t;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  // coord comp
  VI v;
  {
    set<int> vs;
    REP(i, 0, n) {
      vs.insert(a[i]);
    }
    v = vector<int>(vs.begin(), vs.end());
  }
  map<int, int> v_inv;
  int m = v.size();
  REP(i, 0, m) {
    v_inv[v[i]] = i;
  }
  REP(i, 0, n) {
    a[i] = v_inv[a[i]];
  }
  vector<VI> cur(m, VI(m, -1));
  vector<VI> sum(m, VI(m, -1));
  REP(i, 0, m) {
    sum[i][i] = 0;
  }
  REP(i, 0, n) {
    vector<PI> t;
    REP(j, i, n) {
      if (a[i] > a[j]) { continue; }
      t.push_back(PI(a[j], j));
    }
    int p = t.size();
    VI dp(p);
    REP(j, 0, p) {
      int ma = 1;
      REP(k, 0, j) {
	if (t[k].first <= t[j].first) {
	  ma = max(ma, dp[k] + 1);
	}
      }
      dp[j] = ma;
      int &ent = cur[a[i]][t[j].first];
      ent = max(ent, ma);
    }
  }
  REP(i, 0, m) {
    REP(j, i + 1, m) {
      cur[i][j] = max(cur[i][j], cur[i][j - 1]);
    }
  }
  if (0) {
    cerr << "cur:";
    REP(i, 0, m) {
      cerr << "[" << i << "]";
      REP(j, 0, m) {
	cerr << " " << cur[i][j];
      }
      cerr << endl;
    }
  }
  while (t > 0) {
    if (t % 2 == 1) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    t /= 2;
  }
  int ma = 0;
  REP(i, 0, m) {
    REP(j, 0, m) {
      ma = max(ma, sum[i][j]);
    }
  }
  cout << ma << endl;
}
