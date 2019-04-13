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
typedef pair<ll, ll> PL;

ll p;

ll powmod(ll a, ll e, ll mod) {
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
void add(ll &x, ll y, ll mod) {
  if (x + y >= mod) {
    x += y - mod;
  } else {
    x += y;
  }
}
ll sum(ll x, ll y) {
  add(x, y, p);
  return x;
}

void fail(void) {
  cout << "impossible" << endl;
  exit(0);
}

void del(const vector<PL> &vm) {
  int n = vm.size();
  if (n == 1) {
    cout << "0 0 " << vm[0].second << "\n";
    return;
  }
  if (n == 2) {
    ll x0 = vm[0].first, y0 = vm[0].second;
    ll x1 = vm[1].first, y1 = vm[1].second;
    ll a = sum(y1, - y0 + p) * powmod(sum(x1, p - x0), p - 2, p) % p;
    ll b = y0;
    add(b, p - (x0 * a % p), p);
    cout << "0 " << a << " " << b << "\n";
    return;
  }
  ll x1 = sum(vm[1].first, p - vm[0].first);
  ll x2 = sum(vm[2].first, p - vm[0].first);
  ll c = vm[0].second;
  ll y1 = sum(vm[1].second, p - c);
  ll y2 = sum(vm[2].second, p - c);
  ll tap = (x1 * x2 % p) * sum(x1, p - x2) % p;
  tap = powmod(tap, p - 2, p);
  ll a = sum(x2 * y1 % p, p - (x1 * y2 % p)) * tap % p;
  ll b = sum((x2 * y1 % p) * x2 % p, p - ((x1 * y2 % p) * x1 % p)) * tap % p;
  b = b == 0 ? 0 : p - b;
  ll bias = vm[0].first;
  add(c, (bias * bias % p) * a % p, p);
  add(c, p - (b * bias % p), p);
  add(b, p - (2 * a * bias % p), p);
  REP(i, 0, n) {
    ll x = vm[i].first;
    ll est = sum((a * x + b) % p * x % p, c);
    if (est != vm[i].second) fail();
  }
  cout << a << " " << b << " " << c << "\n";
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n >> p;
  VL a(n), b(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  map<ll, ll> m;
  REP(i, 0, n) {
    auto it = m.find(a[i]);
    if (it == m.end()) {
      m[a[i]] = b[i];
    } else {
      if (it->second != b[i]) fail();
    }
  }
  vector<PL> vm;
  for (auto e: m) {
    vm.push_back(e);
  }
  del(vm);
}
