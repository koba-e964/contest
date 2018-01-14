#include <algorithm>
#include <bitset>
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

// [c, d)
string calc_naive(int a, int b, int c, int d) {
  int mi = min(a, b);
  int ma = max(a, b);
  int len = (ma + mi) / (mi + 1);
  //DEBUGP(len);
  // TODO it is O(a + b)
  assert (a + b <= 10000);
  int na = a;
  int nb = b;
  string s;
  char cur = '*';
  int cnt = 0;
  REP(i, 0, a + b) {
    if (cnt >= len) {
      cur = 'A' + 'B' - cur;
      s += cur;
      cnt = 1;
      if (cur == 'A') {
	na -= 1;
      } else {
	nb -= 1;
      }
      continue;
    }
    if (na >= 1 && nb <= na * len) {
      if (cur != 'A') {
	cur = 'A';
	cnt = 0;
      }
      s += cur;
      cnt += 1;
      na -= 1;
      continue;
    }
    if (cur != 'B') {
      cur = 'B';
      cnt = 0;
    }
    s += 'B';
    nb -= 1;
    cnt += 1;
  }
  assert (na == 0);
  assert (nb == 0);
  //DEBUGP(s);
  return s.substr(c, d - c);
}

// [c, d)
string calc(int a, int b, int c, int d) {
  int mi = min(a, b);
  int ma = max(a, b);
  int len = (ma + mi) / (mi + 1);
  //DEBUGP(calc_naive(a, b, 0, a + b));
  // DEBUGP(len);
  int zpw = (a + b) % (len + 1);
  // x s.t. z >= 1?
  ll pass = -1;
  ll fail = a + b;
  while (fail - pass > 1) {
    ll mid = (fail + pass) / 2;
    ll na = a - mid * len;
    ll nb = b - mid;
    if (na >= 1 && nb >= 0 && nb <= na * len) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  if (pass == -1) {
    string ret;
    REP(i, c, d) {
      int j = a + b - 1 - i;
      ret += j % (len + 1) == len ? "A" : "B";
    }
    return ret;
  }
  ll na = a - pass * len;
  ll nb = b - pass;
  string deb;
  REP(i, c, min((ll)d, (len + 1) * pass)) {
    deb += i % (len + 1) == len ? "B" : "A";
  }
  ll y = nb / len;
  ll w = nb - y * len;
  ll z = na - y;
  REP(i, max((ll)c, (len + 1) * pass), min((ll)d, (len + 1) * pass + z)) {
    deb += "A";
  }
  REP(i, max((ll)c, (len + 1) * pass + z), min((ll)d, (len + 1) * pass + z + w)) {
    deb += "B";
  }
  REP(i, max((ll)c, (len + 1) * pass + z + w), d) {
    int j = (a + b - 1 - i) % (len + 1);
    deb += j == len ? "A" : "B";
  }
  return deb;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  if (0) {
    REP(a, 1, 10) {
      REP(b, 1, 10) {
	string naive = calc_naive(a, b, 0, a + b);
	string smart = calc(a, b, 0, a + b);
	if (naive != smart) {
	  DEBUGP(a);
	  DEBUGP(b);
	  DEBUGP(naive);
	  DEBUGP(smart);
	}
      }
    }
  }
  int q;
  cin >> q;
  REP(mulieribus, 0, q) {
    int a, b, c, d;
    cin >> a >> b >> c >> d;
    cout << calc(a, b, c - 1, d) << "\n";
  }
}
