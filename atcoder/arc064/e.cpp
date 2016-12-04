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

double inf = 1.0/0;

int main(void){
  double xs, ys, xt, yt;
  cin >> xs >> ys >> xt >> yt;
  int n;
  cin >> n;
  vector<double> x(n + 2), y(n + 2), r(n + 2);
  REP(i, 0, n) {
    cin >> x[i] >> y[i] >> r[i];
  }
  x[n] = xs;
  y[n] = ys;
  r[n] = 0;
  x[n + 1] = xt;
  y[n + 1] = yt;
  r[n + 1] = 0;
  vector<vector<double> > dist(n + 2, vector<double>(n + 2, inf));
  REP(i, 0, n + 2) {
    dist[i][i] = 0;
  }
  REP(i, 0, n + 2) {
    REP(j, 0, n + 2) {
      double sqdist = pow(x[i] - x[j], 2) + pow(y[i] - y[j], 2);
      double rdist = sqrt(sqdist);
      dist[i][j] = max(0.0, rdist - r[i] - r[j]);
    }
  }
  typedef pair<double, int> PDI;
  priority_queue<PDI, vector<PDI>, greater<PDI> > que;
  que.push(PDI(0.0, n));
  vector<double> d(n + 2, 1.0/0);
  double EPS = 1e-11;
  while (not que.empty()) {
    PDI t = que.top(); que.pop();
    int v = t.second;
    if (d[v] <= t.first + EPS) {
      continue;
    }
    d[v] = t.first;
    REP(i, 0, n + 2) {
      que.push(PDI(d[v] + dist[v][i], i));
    }
  }
  printf("%.15f\n", d[n + 1]);
}
