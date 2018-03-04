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

const int DEBUG = 0;

VI lis(const VI &a) {
  int n = a.size();
  VI m(n + 1);
  VI dp(n);
  int maxl = 0;
  REP(i, 0, n) {
    int pass = 0, fail = maxl + 1;
    while (fail - pass > 1) {
      int mid = (fail + pass) / 2;
      if (a[m[mid]] <= a[i]) {
	pass = mid;
      } else {
	fail = mid;
      }
    }
    dp[i] = pass + 1;
    maxl = max(maxl, pass + 1);
    m[pass + 1] = i;
  }
  if (DEBUG) {
    cerr << "a:";
    REP(i, 0, n) cerr << " " << a[i];
    cerr << endl;
    cerr << "dp:";
    REP(i, 0, n) cerr << " " << dp[i];
    cerr << endl;
  }
  return dp;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI l(n), r(n);
  VI acc(m + 1);
  REP(i, 0, n) {
    cin >> l[i] >> r[i];
    acc[r[i]] += 1;
    acc[l[i] - 1] -= 1;
  }
  for (int i = m - 1; i >= 0; --i) {
    acc[i] += acc[i + 1];
  }
  acc = VI(acc.begin() + 1, acc.end());
  if (1) {
    cerr << "acc:";
    REP(i, 0, m + 1)
      cerr << " " << acc[i];
    cerr << endl;
  }
  VI left = lis(acc);
  reverse(acc.begin(), acc.end());
  VI right = lis(acc);
  reverse(right.begin(), right.end());
  REP(i, 1, m) {
    left[i] = max(left[i], left[i - 1]);
  }
  int ma = 0;
  REP(i, -1, m) {
    ma = max((i == -1 ? 0 : left[i]) + (i < m - 1 ? right[i + 1] : 0), ma);
  }
  cout << ma << "\n";
}
