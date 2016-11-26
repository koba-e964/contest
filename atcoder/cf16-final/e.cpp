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

ll solve(ll n, ll a) {
  ll mi = n;
  if (n == 1) {
    return 1;
  }
  REP(b, 1, 40) {
    if (2 * n < 1LL << b) {
      break;
    }
    ll lo = 1;
    ll hi = n;
    while (hi - lo > 1) {
      ll mid = (hi + lo) / 2;
      if (pow(mid, b) >= n) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    assert (hi >= 2);
    // hi
    int minus = 0;
    REP(i, 0, b) {
      double omni = 1;
      ll scient = 1;
      REP(j, 0, i) { omni *= hi - 1; scient *= hi - 1; }
      REP(j, 0, b - i) { omni *= hi; scient *= hi; }
      if (omni >= n) {
	minus = i;
      }
    }
    ll res = (b - 1) * a + hi * b - minus;
    if (0) {
      cerr << "b=" << b << endl;
      cerr << "hi=" << hi << endl;
      cerr << "minus = " << minus<<endl;
      cerr << "res=" << res << endl << endl;
    }
    mi = min(mi, res);
  }
  return mi;
}

int main(void){
  ll n, a;
  cin >> n >> a;
  cout << solve(n, a) << endl;
}
