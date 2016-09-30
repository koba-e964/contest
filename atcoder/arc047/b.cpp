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
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;
const int N = 100010;

PL mima(const VL &a) {
  ll mi = 1e16;
  ll ma = -1e16;
  REP(i, 0, a.size()) {
    mi = min(mi, a[i]);
    ma = max(ma, a[i]);
  }
  return PL(mi, ma);
}

bool check(VL &x, VL &y, ll sx, ll sy, ll r) {
  int n = x.size();
  if ((sx + sy) % 2 != 0) {
    return false;
  }
  if (abs(sx + sy) > (ll)2e9 || abs(sx - sy) > (ll)2e9) {
    return false;
  }
  REP(i, 0, n) {
    if (max(abs(x[i] - sx), abs(y[i] - sy)) != r) {
      return false;
    }
  }
  return true;
}
int main(void){
  int n;
  cin >> n;
  VL x(n), y(n);
  REP(i, 0, n) {
    ll a, b;
    cin >> a >> b;
    x[i] = a + b;
    y[i] = a - b;
  }
  vector<PL> sol;
  PL mima_x = mima(x);
  PL mima_y = mima(y);
  ll dist_x = mima_x.second - mima_x.first;
  ll dist_y = mima_y.second - mima_y.first;
  ll dist = max(dist_x, dist_y);
  if (dist % 2 == 1) dist++;
  if (dist_x >= dist_y) {
    sol.push_back(PL(mima_x.first + dist / 2, mima_y.first + dist / 2));
    sol.push_back(PL(mima_x.first + dist / 2, mima_y.second - dist / 2));
  }
  if (dist_x < dist_y) {
    sol.push_back(PL(mima_x.first + dist / 2, mima_y.first + dist / 2));
    sol.push_back(PL(mima_x.second - dist / 2, mima_y.first + dist / 2));
  }
  REP(i, 0, sol.size()) {
    REP(dx, -4, 5) {
      REP(dy, -4, 5) {
	PL s = sol[i];
	s.first += dx;
	s.second += dy; // a bit perturbation
	if (check(x, y, s.first, s.second, max(abs(x[0] - s.first), abs(y[0] - s.second)))) {
	  cout << (s.first + s.second) / 2 << " ";
	  cout << (s.first - s.second) / 2 << endl;
	  return 0;
	}
      }
    }
  }
  assert (0);
}
