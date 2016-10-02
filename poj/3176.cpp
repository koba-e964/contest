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

const int  N = 360;

int a[N][N], dp[N][N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    REP(j, 0, i + 1) {
      cin >> a[i][j];
    }
  }
  dp[0][0] = a[0][0];
  REP(i, 1, n) {
    REP(j, 0, i + 1) {
      int ma = 0;
      if (j >= 1) {
	ma = max(ma, dp[i - 1][j - 1]);
      }
      if (j <= i - 1) {
	ma = max(ma, dp[i - 1][j]);
      }
      dp[i][j] = a[i][j] + ma;
    }
  }
  int ma = 0;
  REP(i, 0, n) {
    ma = max(ma, dp[n - 1][i]);
  }
  cout << ma << endl;
}
