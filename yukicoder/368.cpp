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

ll powmod(ll x, ll e) {
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

int main(void){
  int n, k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  set<ll> fbase;
  vector<map<ll, int> > fact(n);
  REP(i, 0, n) {
    ll p = 2;
    ll v = a[i];
    while (v > 1 && p * p <= v) {
      int cnt = 0;
      while (v % p == 0) {
	cnt++;
	v /= p;
      }
      if (cnt != 0) {
	fbase.insert(p);
	fact[i][p] = cnt;
      }
      if (p == 2) {
	p = 3;
      } else {
	p += 2;
      }
    }
    if (v > 1) {
      fbase.insert(v);
      fact[i][v] = 1;
    }
  }
  ll sum = 1;
  for (set<ll>::iterator it = fbase.begin(); it != fbase.end(); ++it) {
    ll p = *it;
    VI exps;
    REP(i, 0, n) {
      if (fact[i].count(p)) {
	exps.push_back(fact[i][p]);
      }
    }
    int cnt = 0;
    sort(exps.rbegin(), exps.rend());
    REP(i, 0, min(k, (int) exps.size())) {
      cnt += exps[i];
    }
    sum = sum * powmod(p, cnt) % mod;
  }
  cout << sum << endl;
}
