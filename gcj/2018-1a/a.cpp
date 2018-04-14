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

bool possible(int r, int c, int h, int v, const vector<VI> &acc) {
  int tot = acc[r][c];
  if (tot % ((h + 1) * (v + 1)) != 0) {
    return false;
  }
  int each = tot / ((h + 1) * (v + 1));
  VI cut_h(h + 2);
  VI col(r + 1);
  REP(i, 0, r + 1) col[i] = acc[i][c];
  REP(i, 0, h) {
    int acc = each * (v + 1) * (i + 1);
    int idx = lower_bound(col.begin(), col.end(), acc) - col.begin();
    if (col[idx] != acc) return false;
    cut_h[i + 1] = idx;
  }
  cut_h[0] = 0; cut_h[h + 1] = r;
  VI cut_v(v + 2);
  VI row(c + 1);
  REP(i, 0, c + 1) row[i] = acc[r][i];
  REP(i, 0, v) {
    int acc = each * (h + 1) * (i + 1);
    int idx = lower_bound(row.begin(), row.end(), acc) - row.begin();
    if (row[idx] != acc) return false;
    cut_v[i + 1] = idx;
  }
  cut_v[0] = 0; cut_v[v + 1] = c;
  REP(i, 0, h + 1) {
    REP(j, 0, v + 1) {
      int chip = acc[cut_h[i + 1]][cut_v[j + 1]]
	- acc[cut_h[i + 1]][cut_v[j]]
	- acc[cut_h[i]][cut_v[j + 1]]
	+ acc[cut_h[i]][cut_v[j]];
      if (chip != each) return false;
    }
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int r, c, h, v;
    cin >> r >> c >> h >> v;
    vector<string> s(r);
    REP(i, 0, r) cin >> s[i];
    vector<VI> acc(r + 1, VI(c + 1, 0));
    REP(i, 0, r) {
      REP(j, 0, c) {
	acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j] + (s[i][j] == '@' ? 1 : 0);
      }
    }
    bool ok = possible(r, c, h, v, acc);
    cout << "Case #" << case_nr << ": " << (ok ? "POSSIBLE" : "IMPOSSIBLE") << endl;
  }
}
