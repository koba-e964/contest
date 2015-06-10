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

const int N = 16;
int dp[1 << N];
int d[N];
int main(void){
  int n;
  cin >> n;
  if (n > N) {
    return 1;
  }
  int lvmask = 0;
  REP(i, 0, n) {
    cin >> d[i];
    if (d[i] < 0) {
      lvmask |= 1 << i;
    }
  }
  dp[0] = 100;
  REP (bits, 1, 1 << n) {
    int ma = 0;
    int lim = 100 * (1 + __builtin_popcount(bits & lvmask));
    REP(i, 0, n) {
      if (bits & (1 << i)) {
	int sub = dp[bits ^ 1 << i];
	if (!sub) { continue; }
	if (d[i] < 0) {
	  ma = max(ma, max(sub + d[i], 0));
	} else {
	  ma = max(ma, min(sub + d[i], lim));
	}
      }
    }
    dp[bits] = ma;
  }
  cout << dp[(1 << n) - 1] << endl;
}
