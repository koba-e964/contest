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

const int N = 10;
double dp[1 << (N + 1)][1 << N] = {};

int main(void){
  int k;
  cin >> k;
  vector<double> elo(1 << k);
  REP(i, 0, 1 << k) {
    cin >> elo[i];
  }
  for (int i = 1 << k; i < 1 << (k + 1); ++i) {
    dp[i][i - (1 << k)] = 1.0;
  }
  for (int i = (1 << k) - 1; i >= 1; --i) {
    // manipulate dp[i]
    // children: 2 * i, 2 * i + 1
    int v = i;
    int sh = 0;
    while ((v & (1 << k)) == 0) {
      v <<= 1;
      sh++;
    }
    v -= 1 << k;
    // range: [v, v + 2^sh)
    REP(j, v, v + (1 << sh)) {
      REP(l, v, v + (1 << sh)) {
	double prob_j = 1.0 / (1.0 + pow(10.0, (- elo[j] + elo[l]) / 400.0));
	double factor = dp[2 * i][j] * dp[2 * i + 1][l];
	dp[i][j] += prob_j * factor;
	dp[i][l] += (1 - prob_j) * factor;
      }
    }
  }
  REP(i, 0, 1 << k) {
    printf("%.10f\n", dp[1][i]);
  }
}
