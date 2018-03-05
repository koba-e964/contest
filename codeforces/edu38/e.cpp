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

const int N = 1001000;
ll fact[N], invfact[N];
void init(void) {
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  invfact[N - 1] = powmod(fact[N - 1], mod - 2);
  for (int i = N - 2; i >= 0; --i) {
    invfact[i] = invfact[i + 1] * (i + 1) % mod;
  }
}

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

ll a[N], acc[N];
ll dpg[N];
int pre[N];

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  sort(a, a + n);
  reverse(a, a + n);
  acc[0] = 0;
  REP(i, 0, n) {
    acc[i + 1] = (acc[i] + a[i]) % mod;
    if (i > 0 && a[i] == a[i - 1]) {
      pre[i] = pre[i - 1];
    } else {
      pre[i] = i;
    }
  }
  dpg[0] = 0;
  ll gfac = 0;
  REP(i, 1, n + 1) {
    ll tmp = gfac * fact[i - 1] % mod;
    add(tmp, acc[i] * fact[i - 1]);
    dpg[i] = tmp;
    if (i == n) {
      add(tmp, a[0] * (mod - fact[n]));
      cout << tmp << "\n";
      break;
    }
    ll tmp2 = dpg[pre[i]] * invfact[pre[i]] % mod;
    add(gfac, tmp2);
  }
  if (0) {
    cerr << "dpg:";
    REP(i, 0, n + 1) cerr << " " << dpg[i];
    cerr << endl;
  }
}
