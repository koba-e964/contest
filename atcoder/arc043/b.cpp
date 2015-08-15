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

const int N = 100100;
int d[N] = {};

const ll mod = 1e9 + 7;

ll dp[5][N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    int q;
    cin >> q;
    d[q]++;
  }
  REP(j, 0, N) {
    dp[0][j] = d[j];
    if (j >= 1) {
      dp[0][j] += dp[0][j - 1];
    }
  }
  REP(i, 1, 4) {
    REP(j, 0, N) {
      ll sum = 0;
      sum = dp[i - 1][j / 2];
      sum *= d[j];
      if (j >= 1) {
	sum += dp[i][j - 1];
      }
      sum %= mod;
      dp[i][j] = sum;
    }
  }
  cout << dp[3][N - 1] << endl;
}
