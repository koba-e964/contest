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

int n, m;
const int N = 1000;
double a[N], b[N], c[N], d[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m;
  REP(i, 0, n) cin >> a[i] >> b[i];
  REP(i, 0, m) cin >> c[i] >> d[i];
  double pass = 0, fail = 1e6;
  REP(_, 0, 100) {
    double mid = (pass + fail) / 2;
    vector<double> diff(n);
    double hlp = -1e8;
    REP(i, 0, n) diff[i] = b[i] - a[i] * mid;
    REP(i, 0, m) hlp = max(hlp, d[i] - mid * c[i]);
    sort(diff.rbegin(), diff.rend());
    double tot = 0;
    REP(i, 0, 4) tot += diff[i];
    tot += max(diff[4], hlp);
    if (tot >= 0) pass = mid;
    else fail = mid;
  }
  cout << fixed << setprecision(15) << pass << endl;
}
