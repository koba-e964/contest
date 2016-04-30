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

const int N = 100;

double x[N], r[N], h[N];

double calc(double a, double b, int i) {
  a -= x[i];
  b -= x[i];
  b = min(b, 0.0);
  a = min(a, 0.0);
  b = max(b, -h[i]);
  a = max(a, -h[i]);
  return acos(-1) / 3.0 * pow(r[i] / h[i], 2) * (pow(b, 3) - pow(a, 3));
}

int main(void){
  int n, q;
  cin >> n >> q;
  REP(i, 0, n) {
    cin >> x[i] >> r[i] >> h[i];
    x[i] += h[i];
  }
  REP(loop_cnt, 0, q) {
    double a, b;
    cin >> a >> b;
    double sum = 0.0;
    REP(i, 0, n) {
      sum += calc(a, b, i);
    }
    printf("%.9f\n", sum);
  }
}
