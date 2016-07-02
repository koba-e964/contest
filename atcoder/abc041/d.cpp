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
int pred[N];
ll dp[1 << N];
int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    pred[y] |= 1 << x;
  }
  dp[0] = 1;
  REP(bits, 1, 1 << n) {
    REP(i, 0, n) {
      if ((bits & (1 << i)) == 0) {
	continue;
      }
      // pred[i] <= bits
      if ((pred[i] & bits) == pred[i]) {
	dp[bits] += dp[bits - (1 << i)];
      }
    }
  }
  cout << dp[(1 << n) - 1] << endl;
}
