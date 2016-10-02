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

const int T = 1010;
int a[T];
int dp[T][2][T];

int main(void){
  int t, w;
  cin >> t >> w;
  REP(i, 0, t) {
    cin >> a[i];
    a[i]--;
  }
  dp[0][0][0] = 0;
  dp[0][1][0] = 0;
  REP(i, 1, t + 1) {
    REP(b, 0, 2) {
      REP(j, 0, i + 1) {
	dp[i][b][j] = dp[i - 1][b][j];
	dp[i][b][j] = max(dp[i][b][j], dp[i - 1][1 - b][j - 1]);
	if (b == a[i - 1]) {
	  dp[i][b][j]++;
	}
      }
    }
  }
  int ma = 0;
  REP(i, 0, w + 1) {
    ma = max(ma, dp[t][0][i]);
    ma = max(ma, dp[t][1][i]);
  }
  cout << ma << endl;
}
