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

void chmax(int &a, int b) {
  a = max(a, b);
}

vector<VI> dp;

void init(int r, int b) {
  dp = vector<VI>(r + 1, VI(b + 1, -1));
  dp[0][0] = 0;
  REP(i, 0, min(35, r) + 1) {
    REP(j, 0, min(35, b) + 1) {
      if (i == 0 && j == 0) continue;
      vector<VI> dp2(dp);
      REP(k, 0, r - i + 1) {
	REP(l, 0, b - j + 1) {
	  if (dp[k][l] < 0) continue;
	  chmax(dp2[k + i][l + j], dp[k][l] + 1);
	}
      }
      swap(dp2, dp);
    }
  }
}
int solve_naive(int r, int b) {
  vector<VI> dp(r + 1, VI(b + 1, -1));
  dp[0][0] = 0;
  REP(i, 0, r + 1) {
    REP(j, 0, b + 1) {
      if (i == 0 && j == 0) continue;
      vector<VI> dp2(dp);
      REP(k, 0, r - i + 1) {
	REP(l, 0, b - j + 1) {
	  if (dp[k][l] < 0) continue;
	  chmax(dp2[k + i][l + j], dp[k][l] + 1);
	}
      }
      swap(dp2, dp);
    }
  }
  return dp[r][b];
}

#if 0
void check(void) {
  REP(r, 1, 50) {
    REP(b, r, 50) {
      if (1) {
	int actual = solve(r, b);
	int naive = solve_naive(r, b);
	if (actual != naive) {
	  cerr << "(r, b) = (" << r << ", " << b << ")" << endl;
	  DEBUGP(actual);
	  DEBUGP(naive);
	  assert(0);
	}
      }
    }
  }
}
#endif

int main(void) {
  // check();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  init(500, 500);
  REP(case_nr, 1, t + 1) {
    int r, b;
    cin >> r >> b;
    cout << "Case #" << case_nr << ": " << dp[r][b] << endl;
  }
}
