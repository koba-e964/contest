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

const int N = 10100;
string s[N];

const int W = 101;
int dp[W][W][W];
const int inf = 1e9;

int main(void){
  int n, r, c;
  cin >> n >> r >> c;
  REP(i, 0, n * r) {
    cin >> s[i];
  }
  REP(i, 0, n) {
    REP(j, 0, r) {
      REP(k, 0, c) {
	dp[i][j][k] = inf;
      }
    }
  }
  for (int i = n - 1; i >= 0; --i) {
    for (int j = r - 1; j >= 0; --j) {
      for (int k = c - 1; k >= 0; --k) {
	int &ret = dp[i][j][k];
	int ret2 = inf;
	if (i == n - 1 && j == r - 1 && k == c - 1) {
	  ret = 0;
	  continue;
	}
	bool hole = s[i * r + j][k] == 'H';
	int t = hole ? 0 : s[i * r + j][k] - '0';
	if (hole) {
	  assert (i < n - 1);
	  ret2 = min(ret2, dp[i + 1][j][k]);
	} else {
	  if (j < r - 1) {
	    ret2 = min(ret2, dp[i][j + 1][k]);
	  }
	  if (k < c - 1) {
	    ret2 = min(ret2, dp[i][j][k + 1]);
	  }
	}
	ret = ret2 == inf ? inf : ret2 + t;
      }
    }
  }
  cout << dp[0][0][0] << endl;
}
