#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 100100;
double dp[N][7];
double tot[N];
double inv2[8];


int main(void) {
  inv2[0] = 1;
  REP(i, 1, 8) inv2[i] = inv2[i - 1] / 2;
  dp[0][0] = 1;
  REP(i, 0, N - 1) {
    REP(j, 0, 7) {
      double p = inv2[j];
      if (j < 7) {
        dp[i + 1][j + 1] += dp[i][j] * p;
        tot[i + 1] += dp[i][j] * p;
      }
      dp[i + 1][0] += dp[i][j] * (1 - p);
    }
    tot[i + 1] += tot[i];
  }
  int n;
  while (cin >> n && n) printf("%.15f\n", tot[n]);
}
