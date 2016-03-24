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
const int LIM = 6000;

double dp[2][5 * LIM + 1];


double calc(int n, ll l, ll r) {
  dp[0][0] = 1.0;
  REP(i, 0, n) {
    int t = i % 2;
    REP(j, 0, 5 * i + 6) {
      dp[1 - t][j] = 0;
    }
    REP(j, 0, 5 * i + 6) {
      REP(k, 0, min(5, j) + 1) {
	dp[1 - t][j] += dp[t][j - k] / 6.0;
      }
    }
  }
  double sum = 0;
  for(ll i = l; i <= r; ++i) {
    sum += dp[n % 2][i];
  }
  return sum;
}

int main(void){
  ll n, l, r;
  cin >> n >> l >> r;
  if (n < LIM) {
    printf("%.15f\n", calc(n, max(l - n, 0LL) , min(r - n, 5 * n)));
  } else {
    double lb = (l - 0.5 - 3.5 * n) / sqrt(35.0 / 6.0 * n);
    double rb = (r + 0.5 - 3.5 * n) / sqrt(35.0 / 6.0 * n);
    double res = 0.5 * (erf(rb) - erf(lb));
    printf("%.15f\n", res);
  }
}
