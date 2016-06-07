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
typedef pair<int, int> PI;
const double EPS=1e-9;

ll mod = 1e9 + 7;
ll inv2 = (mod + 1) / 2;

int main(void){
  ll n, k;
  cin >> n >> k;

  vector<ll> fact;
  for (ll i = 1; i * i <= k; ++i) {
    if (k % i == 0) {
      fact.push_back(i);
      if (k != i * i) {
	fact.push_back(k / i);
      }
    }
  }
  sort(fact.begin(), fact.end());
  int nf = fact.size();

  vector<ll> coef(nf, 0LL);

  REP(i, 0, nf) {
    coef[i] = k / fact[i];
  }

  REP(i, 0, nf) {
    REP(j, i + 1, nf) {
      if (fact[j] % fact[i] == 0) {
	coef[j] += mod - coef[i];
	coef[j] %= mod;
      }
    }
  }

  ll sum = 0;

  REP(i, 0, nf) {
    ll till = n / fact[i];
    ll cur = till * (till + 1) % mod;
    cur = cur * inv2 % mod;
    cur = cur * fact[i] % mod;
    cur = cur * coef[i] % mod;
    sum += cur;
    sum %= mod;
  }
  cout << sum << endl;
}
