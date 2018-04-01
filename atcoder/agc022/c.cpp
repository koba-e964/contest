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

void fail(void) {
  cout << -1 << endl;
  exit(0);
}

ll calc(ll pat, int k) {
  int r = 51 / k;
  ll x = pat;
  ll y = 0;
  REP(i, 0, r + 1) {
    y |= x & ((1LL << k) - 1);
    x >>= k;
  }
  return pat | y;
}


bool poss(ll pat, ll goal, int ma) {
  for (int i = ma; i >= 1; --i) {
    pat = calc(pat, i);
  }
  return (pat & goal) != 0;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  ll ans = 0;
  VL apat(n), bpat(n);
  REP(i, 0, n) {
    apat[i] = 1LL << a[i];
    bpat[i] = 1LL << b[i];
  }
  for (int i = 49; i >= 1; --i) {
    bool ok = true;
    REP(j, 0, n) {
      if (not poss(apat[j], bpat[j], i - 1)) {
	ok = false;
	break;
      }
    }
    if (not ok) {
      ans |= 1LL << i;
      REP(j, 0, n) {
	apat[j] = calc(apat[j], i);
      }
    }
  }
  // check
  REP(i, 0, n) {
    if ((apat[i] & bpat[i]) == 0) {
      fail();
    }
  }
  cout << ans << endl;
}
