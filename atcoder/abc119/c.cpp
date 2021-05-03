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
const ll mod = 1e9 + 7;

int bc(int x) {
  return __builtin_popcount(x);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, a, b, c;
  cin >> n >> a >> b >> c;
  VL l(n);
  REP(i, 0, n) cin >> l[i];
  VI p(n);
  REP(i, 0, n) p[i] = i;
  ll mi = 1e18;
  REP(s, 1, 1 << n) {
    REP(t, 1, 1 << n) {
      if (s & t) continue;
      REP(u, 1, 1 << n) {
        if (s & u) continue;
        if (t & u) continue;
        ll tmp = 0;
        ll k = 0;
        REP(i, 0, n) if (s & 1 << i) k += l[i];
        tmp += abs(k - a);
        k = 0;
        REP(i, 0, n) if (t & 1 << i) k += l[i];
        tmp += abs(k - b);
        k = 0;
        REP(i, 0, n) if (u & 1 << i) k += l[i];
        tmp += abs(k - c);
        tmp += 10 * (bc(s) + bc(t) + bc(u) - 3);
        mi = min(mi, tmp);
      }
    }
  }
  cout << mi << endl;
}
