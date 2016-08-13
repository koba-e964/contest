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
typedef pair<int, int> PI;
const double EPS=1e-9;

const ll mod = 1e9 + 7;

const int N = 5010;
ll dp[2][N];

int main(void){
  int n;
  string s;
  cin >> n >> s;
  assert (n < N);
  dp[0][0] = 1;
  REP(i, 0, n) {
    int t = (i + 1) % 2;
    dp[t][0] = dp[1-t][0] + 2 * dp[1-t][1];
    dp[t][0] %= mod;
    REP(j, 1, N) {
      dp[t][j] = dp[1-t][j - 1];
      if (j < N - 1) {
	dp[t][j] += 2 * dp[1-t][j + 1];
      }
      dp[t][j] %= mod;
    }
  }
  cout << dp[n % 2][s.length()] << endl;
}
