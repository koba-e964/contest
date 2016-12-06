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

ll solve(const VI &e) {
  int n = e.size();
  vector<VI> ama(n, VI(n));
  vector<VI> ami(n, VI(n));
  REP(i, 0, n) {
    int acc_ma = 0;
    int acc_mi = n + 1;
    REP(j, i, n) {
      acc_ma = max(acc_ma, e[j]);
      acc_mi = min(acc_mi, e[j]);
      ama[i][j] = acc_ma;
      ami[i][j] = acc_mi;
    }
  }
  ll tot = 0;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      int score = 0;
      int u = e[i];
      int v = e[j];
      int ma = i == n - 1 || j == 0 ? 0 : ama[i + 1][j - 1];
      int mi = i == n - 1 || j == 0 ? 0 : ami[i + 1][j - 1];
      int left_ma = i == 0 ? u : ama[0][i - 1];
      int left_mi = i == 0 ? u : ami[0][i - 1];
      int right_ma = j == n - 1 ? v : ama[j + 1][n - 1];
      int right_mi = j == n - 1 ? v : ami[j + 1][n - 1];
      if (u > v) {
	swap(u, v);
	swap(left_ma, right_ma);
	swap(left_mi, right_mi);
      }
      if (j - i >= 2) {
	if (ma > v) {
	  score = ma;
	} else if (mi < u) {
	  score = v;
	}
      }

      if (left_ma > u) {
	score = max(score, max(left_ma, v));
      }
      if (right_mi < v) {
	score = max(score, v);
      }
      tot += score;
    }
  }
  return tot;
}


int main(void){
  int n, m;
  cin >> n >> m;
  int maxi = -1;
  ll ma = -1;
  REP(i, 0, m) {
    VI e(n);
    REP(j, 0, n) { cin >> e[j]; }
    ll res = solve(e);
    if (res > ma) {
      ma = res;
      maxi = i;
    }
  }
  cout << maxi << endl;
}
