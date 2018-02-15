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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VL a(n), b(m);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, m) {
    cin >> b[i];
  }
  ll mi = 1e18;
  REP(i, 0, n) {
    ll ma = -1e18;
    REP(j, 0, n) {
      if (i == j) continue;
      REP(k, 0, m) {
	ma = max(ma, a[j] * b[k]);
      }
    }
    mi = min(mi, ma);
  }
  cout << mi << "\n";
}
