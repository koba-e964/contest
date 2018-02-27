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

const int N = 1000100;
int ans[N];
int dp[10][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  REP(i, 1, N){
    if (i < 10) {
      ans[i] = i;
    } else {
      int v = i;
      int prod = 1;
      while (v > 0) {
	if (v % 10) {
	  prod *= v % 10;
	}
	v /= 10;
      }
      ans[i] = ans[prod];
    }
  }
  REP(i, 0, N - 1) {
    REP(j, 0, 10) {
      dp[j][i + 1] = dp[j][i];
    }
    dp[ans[i]][i + 1] += 1;
  }
  int q;
  cin >> q;
  REP(i, 0, q) {
    int l, r, k;
    cin >> l >> r >> k;
    cout << dp[k][r + 1] - dp[k][l] << "\n";
  }
}
