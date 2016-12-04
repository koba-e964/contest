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
  x %= mod;
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
  ll n, k;
  cin >> n >> k;
  VL n_fact;
  for (ll i = 1; i * i <= n; ++i) {
    if (n % i == 0) {
      n_fact.push_back(i);
      if (i * i != n) {
	n_fact.push_back(n / i);
      }
    }
  }
  sort(n_fact.begin(), n_fact.end());
  int m = n_fact.size();
  VL sym(m, 0LL); // the number of arrays with symmetry k_fact[i]Z/kZ
  REP(i, 0, m) {
    int block = n_fact[i];
    sym[i] = powmod(k, (block + 1) / 2);
  }
  REP(i, 0, m) {
    REP(j, i + 1, m) {
      if (n_fact[j] % n_fact[i] == 0) {
	sym[j] = (sym[j] - sym[i] + mod) % mod;
      }
    }
  }
  ll tot = 0;
  REP(i, 0, m) {
    ll tmp = sym[i];
    ll f = n_fact[i];
    tmp *= f % 2 ? f : f / 2;
    tmp %= mod;
    tot = (tmp + tot) % mod;
  }
  cout << tot << endl;
}
