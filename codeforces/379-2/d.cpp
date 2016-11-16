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

const int N = 500000;
char t[N];
ll x[N], y[N];

ll check(int dx, int dy, ll x, ll y) {
  if (dy == 0) {
    swap(dx, dy);
    swap(x, y);
  }
  if (dx == 0) {
    if (x != 0) return -1;
    return y / dy <= 0 ? -1 : y / dy;
  }
  ll a = x / dx;
  ll b = y / dy;
  if (a != b || a <= 0) {
    return -1;
  }
  return a;
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  ll x0, y0;
  cin >> x0 >> y0;
  REP(i, 0, n) {
    cin >> t[i] >> x[i] >> y[i];
  }
  bool checked = false;
  REP(dx, -1, 2) {
    REP(dy, -1, 2) {
      if (dx == 0 && dy == 0) { continue; }
      ll mi = 1e16;
      char minc = '*';
      REP(i, 0, n) {
	ll res = check(dx, dy, x[i] - x0, y[i] - y0);
	if (res >= 0 && res < mi) {
	  mi = res;
	  minc = t[i];
	}
      }
      if ((dx + dy + 2) % 2 == 0) {
	checked |= minc == 'B' || minc == 'Q';
      } else {
	checked |= minc == 'R' || minc == 'Q';
      }
    }
  }
  cout << (checked ? "YES" : "NO") << "\n";
}
