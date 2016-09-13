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


const int N = 51;
const int W = 5001;
ll dp[N][W];
ll x[N];

int main(void){
  int n, a;
  cin >> n >> a;
  REP(i, 0, n) {
    cin >> x[i];
    x[i] -= a;
    assert (x[i] >= -50);
  }
  dp[0][2500] = 1;
  REP(i, 1, n + 1) {
    REP(j, 0, W) {
      dp[i][j] += dp[i - 1][j];
      if (j >= x[i - 1] && j - x[i - 1] < W) {
	dp[i][j] += dp[i - 1][j - x[i - 1]];
      }
    }
  }
  cout << dp[n][2500] - 1 << endl;
}
