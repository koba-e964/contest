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


ll totient(ll x) {
  ll ret = 1;
  long long p = 2;
  ll v = x;
  while (v > 1 && p * p <= v) {
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    if (cnt > 0) {
      REP(i, 0, cnt - 1) {
	ret *= p;
      }
      ret *= p - 1;
    }
    p += p == 2 ? 1 : 2;
  }
  if (v > 1) {
    ret *= v - 1;
  }
  return ret;
}

ll calc(ll n, ll k) {
  ll cur = n;
  // This loop will not continue so long; cur decreases quickly.
  for (ll i = 0; i < k; ++i) {
    ll res = totient(cur);
    cur = res;
    if (cur == 1) {
      break;
    }
  }
  return cur % mod;
}


int main(void){
  ll n, k;
  cin >> n >> k;
  k = (k + 1) / 2;
  // I want f^k(n)!
  cout << calc(n, k) << endl;
}
