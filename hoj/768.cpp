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
typedef pair<int, ll> PIL;
const ll mod = 1e9 + 7;

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int N = 123456;
ll fact[N];

void init(void) {
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
}

int main(void){
  init();
  int n;
  cin >> n;
  vector<ll> tbl(10, 0);
  VI cnt(10, 0);
  PIL nonzero(10, 0);
  int dup = 0;
  REP(i, 0, n) {
    int a;
    ll b;
    cin >> a >> b;
    tbl[a] += b;
    cnt[a] += 1;
    if (a != 0 && nonzero >= PIL(a, b)) {
      if (nonzero == PIL(a, b)) {
	dup += 1;
      } else {
	dup = 1;
      }
      nonzero = min(nonzero, PIL(a, b));
    }
  }
  if (nonzero.first == 10) {
    // 0
    cout << 0 << endl << 1 << endl;
    return 0;
  }
  vector<PIL> vec;
  if (cnt[0] > 0) {
    tbl[nonzero.first] -= nonzero.second;
    cnt[nonzero.first] -= 1;
    vec.push_back(nonzero);
  }
  REP(i, 0, 10) {
    vec.push_back(PIL(i, tbl[i]));
  }
  ll sum = 0;
  REP(i, 0, vec.size()) {
    int dig = vec[i].first;
    ll e = vec[i].second;
    ll qm = ((powmod(10, e) + mod - 1) % mod) * powmod(9, mod - 2) % mod;
    qm = qm * dig % mod;
    sum = sum * powmod(10, e) % mod;
    sum = (sum + qm) % mod;
  }
  cout << sum << endl;

  ll nways = cnt[0] > 0 ? dup : 1;
  REP(i, 0, 10) {
    nways *= fact[cnt[i]];
    nways %= mod;
  }
  cout << nways << endl;
}
