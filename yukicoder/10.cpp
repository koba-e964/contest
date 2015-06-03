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


const int N = 52;
const int T = 100100;
int a[N];
int dp[N][T] = {};

int main(void){
  int n;
  cin >> n;
  int tot;
  cin >> tot;
  REP(i, 0, n) {
    cin >> a[i];
  }
  dp[n][tot] = 1;
  for (int i = n - 1; i >= 1; --i) {
    REP(j, 0, T) {
      int p = j + a[i];
      int q = j * a[i];
      if (p < T && dp[i + 1][p]) {
	dp[i][j] = 1;
      }
      if (q < T && dp[i + 1][q]) {
	dp[i][j] = 1;
      }
    }
  }
  if (dp[1][a[0]] == 0) {
    cout << "fail" << endl;
    return 1;
  }
  int cur = a[0];
  REP(i, 1, n) {
    if (dp[i + 1][cur + a[i]]) {
      cout << "+";
      cur += a[i];
      continue;
    }
    cout << "*";
    cur *= a[i];
  }
  cout << endl;
}
