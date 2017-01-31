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


const int N = 110;
const int inf = 1e8;
int dp[N][8];

int main(void){
  int n, m;
  cin >> n >> m;
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  vector<VI> kind(n, VI(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      char c = s[i][j];
      if ('0' <= c && c <= '9') {
	kind[i][j] = 0;
      }
      if ('a' <= c && c <= 'z') {
	kind[i][j] = 1;
      }
      if (c == '#' || c == '*' || c == '&') {
	kind[i][j] = 2;
      }
    }
  }
  REP(i, 0, n + 1) {
    REP(j, 0, 8) {
      dp[i][j] = inf;
    }
  }
  dp[0][0] = 0;
  REP(i, 0, n) {
    REP(bits, 0, 8) {
      REP(j, 0, m) {
	int cost = min(j, m - j);
	int nb = bits | 1 << kind[i][j];
	dp[i + 1][nb] = min(dp[i + 1][nb], cost + dp[i][bits]);
      }
    }
  }
  cout << dp[n][7] << endl;
}
