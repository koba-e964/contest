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
  int n;
  ll l;
  cin >> n >> l;
  VL c(n);
  REP(i, 0, n) {
    cin >> c[i];
    if (i >= 1 && c[i] > 2 * c[i - 1]) {
      c[i] = 2 * c[i - 1];
    }
  }
  ll mi = 1e18;
  while (l > 0 && l <= 1LL << 32) {
    ll tot = 0;
    REP(i, 0, n) {
      if (l & (1LL << i)) {
	tot += c[i];
      }
    }
    tot += (l >> n) * 2 * c[n - 1];
    mi = min(mi, tot);
    l += l & (-l);
  }
  cout << mi << "\n";
}
