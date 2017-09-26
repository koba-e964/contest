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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


ll s_lt(ll);

map<ll, ll> memo;
map<ll, ll> memo_lt;

ll s(ll n) {
  if (n == 0) {
    return 1;
  }
  if (memo.count(n)) {
    return memo[n];
  }
  ll &ret = memo[n];
  ll c = 1;
  while (c <= n) {
    c *= 2;
  }
  c /= 2;
  ll r = n - c;
  return ret = (s(c - 1) + s(r) + s_lt(n) + mod - s_lt(c - 1)) % mod;
}

ll s_lt(ll n) {
  if (n <= 1) {
    return 0;
  }
  if (memo_lt.count(n)) {
    return memo_lt[n];
  }
  ll &ret = memo_lt[n];
  ll c = 1;
  while (c <= n) {
    c *= 2;
  }
  c /= 2;
  ll r = n - c;
  ll sub = s_lt(c / 2 - 1);
  if (n >= c + c / 2) {
    return ret = (s_lt(c + c / 2 - 1) + s_lt(r) + mod - sub) % mod;
  }
  return ret = (s_lt(c - 1) + s_lt(r + c / 2) + s(r) + mod - sub) % mod;
}


int main(void) {
  ll n;
  cin >> n;
  cout << s(n) << endl;
}
