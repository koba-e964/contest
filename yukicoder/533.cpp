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

const int N = 1e6 + 10;
ll dp[3][N];

int main(void){
  int n;
  cin >> n;
  REP(i, 1, n + 1) {
    REP(j, 0, 3) {
      ll tmp = 0;
      REP(k, 0, 3) {
	if (j != k && i > j + 1) {
	  tmp += dp[k][i - j - 1];
	}
      }
      if (i == j + 1) {
	tmp += 1;
      }
      dp[j][i] = tmp % mod;
    }
  }
  ll tot = 0;
  REP(i, 0, 3) {
    tot = (tot + dp[i][n]) % mod;
  }
  cout << tot << endl;
}
