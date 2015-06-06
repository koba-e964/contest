#include <algorithm>
#include <cstdio>
#include <iostream>
#include <vector>
#include <cassert>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;
const ll inf = 0x3fffffffLL << 31;

const int N = 15;
int x[N], y[N];
double w[N];
int n;
double dp[2 << N][N + 1];

// the shortest path from v to 0 using vertices bits
double rec(int bits, int v) {
  double &ret = dp[bits][v];
  if (ret >= 0) {
    return ret;
  }
  assert ((bits & (1 << v)) == 1 << v);
  if (bits == 1 << v) {
    double c = 100 * (abs(x[0] - x[v]) + abs(y[0] - y[v]));
    return ret = c;
  }
  double mi = inf;
  double tot_w = 0;
  REP (i, 1, n + 1) {
    if (i != v && (bits & (1 << i))) {
      tot_w += w[i];
    }
  }
  REP(i, 1, n + 1) {
    if ((bits & (1 << i)) == 0 || v == i) {
      continue;
    }
    double c = (tot_w + 100) * (abs(x[v] - x[i]) + abs(y[v] - y[i]));
    mi = min(mi, c + rec(bits ^ (1 << v), i));
  }
  return ret = mi;
}

int main(void){
  cin >> x[0] >> y[0];
  cin >> n;
  double tot_w = 0;
  REP(i, 1, n+1) {
    cin >> x[i] >> y[i] >> w[i];
    tot_w += w[i];
  }
  double mi = inf;
  REP (i, 0, 2 << N) {
    fill_n(dp[i], N + 1, -1);
  }
  REP(i, 1, n + 1) {
    double c = (tot_w + 100) * (abs(x[0] - x[i]) + abs(y[0] - y[i]));
    mi = min(mi, c + rec((2 << n) - 2, i));
  }
  printf("%.9f\n", (double)mi / 120.0 + tot_w);
}
