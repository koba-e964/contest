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
  ll n;
  int k;
  cin >> n >> k;
  int bc = __builtin_popcountll(n);
  if (bc > k) {
    cout << "No\n";
    return 0;
  }
  ll pass = 64;
  ll fail = -30;
  while (pass - fail > 1) {
    int mid = (pass + fail + 128) / 2 - 64;
    bool ok = false;
    if (mid < 0) {
      ok = n <= (ll)k >> (-mid);
    } else {
      ll cnt = 0;
      cnt = n >> mid;
      cnt += __builtin_popcountll(n & ((1LL << mid) - 1));
      ok = cnt <= k;
    }
    if (ok) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  VI out(k);
  ll r = n;
  int h = pass;
  if (h < 0) {
    r <<= -h;
  }
  REP(i, 0, k) {
    if (h < 0) {
      if (i == k - 1 || r != 1) {
	out[i] = h;
	r -= 1;
      } else {
	out[i] = h - 1;
	h--;
	r *= 2;
	r -= 1;
      }
      continue;
    }
    assert (r >= 0);
    while (r < 1LL << h) { h--; }
    if (i == k - 1 || r != 1LL << h) {
      out[i] = h;
      r -= 1LL << h;
    } else {
      out[i] = h - 1;
      h--;
      if (h < 0) {
	r *= 2;
	r -= 1;
      } else {
	r -= 1LL << h;
      }
    }
  }
  assert (r == 0);
  cout << "Yes\n";
  REP(i, 0, k) {
    cout << out[i] << (i == k - 1 ? "\n" : " ");
  }
}
