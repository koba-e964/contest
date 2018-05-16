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

const int N = 5010;
int dp[N][N], dp2[N][N];

void calc_f(const VI &a) {
  int n = a.size();
  REP(i, 0, n) dp[i][i] = a[i];
  REP(s, 1, n) {
    REP(i, 0, n - s) {
      int j = s + i;
      dp[i][j] = dp[i][j - 1] ^ dp[i + 1][j];
    }
  }
}

void test_calc_f(void) {
  VI a(3);
  a[0] = 8;
  a[1] = 4;
  a[2] = 1;
  calc_f(a);
  assert (dp[0][0] == 8);
  assert (dp[0][1] == 12);
  assert (dp[0][2] == 9);
  assert (dp[1][2] == 5);
  assert (dp[2][2] == 1);
}

void calc_dp2(void) {
  REP(i, 0, N) dp2[i][i] = dp[i][i];
  REP(s, 1, N) {
    REP(i, 0, N - s) {
      int j = i + s;
      dp2[i][j] = max(max(dp2[i][j - 1], dp2[i + 1][j]), dp[i][j]);
    }
  }
}

void test_calc_dp2(void) {
  REP(i, 0, N) REP(j, 0, N) dp[i][j] = 0;
  dp[0][0] = 1;
  dp[0][1] = 4;
  dp[0][2] = 3;
  calc_dp2();
  assert (dp2[0][0] == 1);
  assert (dp2[0][1] == 4);
  assert (dp2[0][2] == 4);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  //test_calc_f();
  //test_calc_dp2();
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  calc_f(a);
  calc_dp2();
  int q;
  cin >> q;
  REP(i, 0, q) {
    int l, r;
    cin >> l >> r;
    l--, r--;
    cout << dp2[l][r] << "\n";
  }
}
