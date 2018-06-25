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

const int DEBUG = 0;


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;


const int A = 7;

const ll mod = 1e9 + 7;

template<ll mod = (ll)1e9 + 7>
struct ModInt {
  ll x;
  ModInt(void): x(0) {}
  ModInt(ll x): x(x % mod){}
  ModInt(const ModInt &x): x(x.x) {}
  ModInt operator+(ModInt o) const {
    ll y = x + o.x;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod);
  }
  ll to_ll() const {
    return x;
  }
  bool operator<(ModInt o) const {
    return x < o.x;
  }
  ModInt pow(ll e) {
    assert (e >= 0);
    ModInt sum = 1;
    ModInt cur = *this;
    while (e > 0) {
      if (e % 2) {
        sum = sum * cur;
      }
      cur = cur * cur;
      e /= 2;
    }
    return sum;
  }
  ModInt inv(void) {
    return pow(mod - 2);
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string s;
  cin >> n >> s;
  int p = 0;
  VI pos(n + 1);
  mt19937 mt;
  vector<ModInt<> > b(A);
  
  REP(c, 0, A) {
    do {
      b[c] = mt();
    } while (b[c].to_ll() == 0);
  }
  vector<vector<ModInt<> > > hsh(A, vector<ModInt<> >(n + 1));
  ModInt<> inv[A] = { 0 };
  REP(c, 0, A) inv[c] = b[c].inv();
  REP(c, 0, A) {
    p = 0;
    ModInt<> cur = 1;
    REP(i, 0, n) {
      ModInt<> tmp = hsh[c][i];
      if (s[i] == '>') {
        p++;
        cur = cur * b[c];
      }
      if (s[i] == '<') {
        p--;
        cur = cur * inv[c];
      }
      if (s[i] == '+') {
        tmp = tmp + cur;
      }
      if (s[i] == '-') {
        tmp = tmp - cur;
      }
      hsh[c][i + 1] = tmp;
      pos[i + 1] = p;
    }
  }
  if (DEBUG) {
    REP(i, 0, n + 1) {
      cerr << i << " " << hsh[0][i] << endl;
    }
  }
  vector<ModInt<> > meguru(A);
  REP(c, 0, A) meguru[c] = hsh[c][n];
  map<vector<ModInt<> >, int> kirika;
  kirika[meguru] = 1;
  ll tot = 0;
  vector<ModInt<> > cur(A);
  REP(c, 0, A) cur[c] = b[c].pow(mod - 1 + pos[n]);
  for (int i = n - 1; i >= 0; --i) {
    if (DEBUG) {
      DEBUGP(i);
      DEBUGP(pos[i]);
    }
    if (pos[i + 1] - pos[i] == 1) {
      REP(c, 0, A) cur[c] = cur[c] * inv[c];
    }
    else if (pos[i + 1] - pos[i] == -1) {
      REP(c, 0, A) cur[c] = cur[c] * b[c];
    }
    vector<ModInt<> > ken(A);
    REP(c, 0, A) ken[c] = cur[c] * meguru[c] + hsh[c][i];
    tot += kirika[ken];
    vector<ModInt<> > tt(A);
    REP(c, 0, A) tt[c] = hsh[c][i];
    kirika[tt] += 1;
  }
  cout << tot << endl;
}
