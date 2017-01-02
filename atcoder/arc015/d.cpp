#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
const double EPS=1e-9;


// After reading http://kmjp.hatenablog.jp/entry/2013/10/06/1000
int main(void){
  int t, n;
  double p;
  cin >> t >> n >> p;
  vector<double> q(n);
  VI x(n), tt(n);
  const int T = 100010;
  vector<double> xt_sum(T);
  REP(i, 0, n) {
    cin >> q[i] >> x[i] >> tt[i];
    xt_sum[tt[i]] += (x[i] - 1) * q[i] * p;
  }
  double sum = 0;
  double cur = 1;
  vector<double> m(T);
  m[T - 1] = 1;
  for (int i = T - 2; i >= 0; --i) {
    m[i] = m[i + 1] + xt_sum[i];
  }
  REP(i, 0, t) {
    sum += cur;
    cur *= m[i + 1];
  }
  printf("%.15f\n", sum);
}
