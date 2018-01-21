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

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int W = 1100;

ll dp[2][W][W]; // [equal to n][pos][popcount]

// {v | v <= s, |v| = k} for each k
void calc(const string &s) {
  int n = s.length();
  dp[1][0][0] = 1;
  REP(i, 0, n) {
    int b = s[i] == '1';
    REP(j, 0, n + 1) {
      if (b == 0) {
	add(dp[0][i + 1][j], dp[0][i][j]);
	add(dp[0][i + 1][j + 1], dp[0][i][j]);
	add(dp[1][i + 1][j], dp[1][i][j]);
      } else {
	add(dp[0][i + 1][j], dp[0][i][j]);
	add(dp[0][i + 1][j + 1], dp[0][i][j]);
	add(dp[0][i + 1][j], dp[1][i][j]);
	add(dp[1][i + 1][j + 1], dp[1][i][j]);
      }
    }
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string n;
  int k;
  cin >> n >> k;
  if (k == 0) {
    cout << "1\n";
    return 0;
  }
  // enumerate [k-1]
  VI tbl(W, -1);
  tbl[1] = 0;
  REP(i, 2, W) {
    tbl[i] = tbl[__builtin_popcount(i)] + 1;
  }
  ll tot = 0;
  calc(n);
  REP(i, 1, W) {
    if (tbl[i] == k - 1) {
      add(tot, dp[0][n.length()][i]);
      add(tot, dp[1][n.length()][i]);
    }
  }
  if (k == 1) {
    add(tot, mod - 1);
  }
  cout << tot << "\n";
}
