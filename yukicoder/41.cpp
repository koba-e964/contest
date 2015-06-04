#include <algorithm>
#include <bitset>
#include <cassert>
#include <deque>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const ll mod = 1e9 + 9;

const int N = 10, M = 100000;

ll dp[N][M];
int main(void){
  int t;
  cin >> t;
  REP(j, 0, M) {
    dp[0][j] = 1;
  }
  REP (i, 1, N) {
    REP(j, 0, M) {
      ll s = 0;
      if (j >= i) {
	s += dp[i][j - i];
      }
      s += dp[i - 1][j];
      s %= mod;
      dp[i][j] = s;
    }
  }
  REP (trial, 0, t) {
    ll m;
    cin >> m;
    cout << dp[9][m / 111111] << endl;
  }
}
