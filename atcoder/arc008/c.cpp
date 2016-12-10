#include <algorithm>
#include <cmath>
#include <cstdio>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 1001;
double dist[N][N];
double x[N], y[N], t[N], r[N];

int main(void){
  REP(i, 0, N) {
    REP(j, 0, N) {
      dist[i][j] = 1.0 / 0;
    }
  }
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> x[i] >> y[i] >> t[i] >> r[i];
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      double d = sqrt(pow(x[i] - x[j], 2) + pow(y[i] - y[j], 2));
      dist[i][j] = d / min(t[i], r[j]);
    }
  }
  REP(k, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  vector<double> sol(n);
  REP(i, 0, n) { sol[i] = dist[0][i]; }
  sort(sol.rbegin(), sol.rend());
  double ma = 0;
  REP(i, 0, n - 1) {
    ma = max(ma, sol[i] + i);
  }
  printf("%.15f\n", ma);
}
