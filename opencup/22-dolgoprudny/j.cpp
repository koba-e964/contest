#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <map>
#include <set>
#include <utility>
#include <vector>
#define REP(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(x) (x).begin(), (x).end()
#define HHH(x) cerr << "L" << __LINE__ << ": " << #x << " = " << (x) << endl

template <typename T> T &chmin(T &a, const T &b) { return a = std::min(a, b); }
template <typename T> T &chmax(T &a, const T &b) { return a = std::max(a, b); }

using ll = long long;
using ld = long double;

using namespace std;

int primes[1024000];

void eratosthenes() {
  const int M = 1001000;
  primes[0] = -1;
  primes[1] = -1;
  for (int i = 2; i * i < M; ++i) {
    if (primes[i] == 0) {
      for (int j = i * i; j < M; j += i) {
        if (primes[j] == 0) primes[j] = i;
      }
    }
  }
}

ll inv(ll a, ll p) {
  return (a == 1 ? 1 : (1 - p * inv(p % a, a)) / a + p);
}

template <int M> class Modulo {
  int n;

public:
  Modulo() : n(0) { ; }
  Modulo(int m) : n(m) {
    if (n >= M)
      n %= M;
    else if (n < 0)
      n = (n % M + M) % M;
  }
  Modulo(ll m) {
    if (m >= M)
      m %= M;
    else if (m < 0)
      m = (m % M + M) % M;
    n = m;
  }
  explicit operator int() const { return n; }
  explicit operator ll() const { return n; }
  bool operator==(const Modulo &a) const { return n == a.n; }
  Modulo &operator+=(const Modulo &a) {
    n += a.n;
    if (n >= M) n -= M;
    return *this;
  }
  Modulo &operator-=(const Modulo &a) {
    n -= a.n;
    if (n < 0) n += M;
    return *this;
  }
  Modulo &operator*=(const Modulo &a) {
    n = (ll(n) * a.n) % M;
    return *this;
  }
  Modulo operator+(const Modulo &a) const {
    Modulo res = *this;
    return res += a;
  }
  Modulo operator-(const Modulo &a) const {
    Modulo res = *this;
    return res -= a;
  }
  Modulo operator-() const { return Modulo(0) - *this; }
  Modulo operator*(const Modulo &a) const {
    Modulo res = *this;
    return res *= a;
  }
  Modulo operator^(ll m) const {
    if (m == 0) return Modulo(1);
    const Modulo a = *this;
    Modulo res = (a * a) ^ (m / 2);
    return m % 2 ? res * a : res;
  }
  Modulo operator/(const Modulo &a) const {
    return *this * inv(ll(a), M);
  }
  Modulo operator/=(const Modulo &a) {
    return *this *= inv(ll(a), M);
  }
};

using Mod = Modulo<998244353>;

vector<int> backet[1024000];
int cnt[1024000];

int gcd(int x, int y) {
  while (y) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

int main() {
  eratosthenes();
  cin.tie(nullptr);
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<int> x(n), y(n);
  Mod res = 1;
  REP(i,n) {
    cin >> x[i] >> y[i];
    int g = gcd(x[i], y[i]);
    x[i] /= g; y[i] /= g;

    int div = y[i];
    while (primes[div] > 1) {
      int d = primes[div];
      int dd = 1;
      while (div % d == 0) {
        ++cnt[dd];
        div /= d;
        dd *= d;
      }
      backet[dd].push_back(i);
    }
    if (div > 1) backet[div].push_back(i);
    res *= (Mod(y[i]) ^ (n - 1));
  }
  // REP(i,20) {
  //   cout << i << ": ";
  //   for (int v: backet[i]) cout << v << " ";
  //   cout << endl;
  // }
  // cout << int(res) << endl;
  Mod rev = 1;
  REP(div,1000000) {
    if (backet[div].empty()) continue;
    rev *= Mod(div) ^ (ll(cnt[div]) * ll(backet[div].size()));
    vector<int> xs;
    for (int id: backet[div]) {
      xs.push_back((x[id] * inv(y[id] / div, div)) % div);
    }
    sort(ALL(xs));
    xs.push_back(-1);
    ll last = 0;
    int p = primes[div] == 0 ? div : primes[div];
    // cerr << "div = " << div << " p = " << p << endl;
    int nowdiv = div;
    while (1) {
      int seq = 0, before = -1;
      ll now = 0;
      for (int v: xs) {
        // cout << v << " ";
        if (v == before) ++seq;
        else {
          ll val = seq * ll(seq - 1);
          now += val;
          before = v;
          seq = 1;
        }
      }
      // cerr << nowdiv << " " << last << " " << now << endl;
      rev *= (Mod(nowdiv) * Mod(div)) ^ ((now - last) / 2);
      if (nowdiv == 1) break;
      nowdiv /= p;
      last = now;
      for (int i = 0; i < (int) xs.size() - 1; i++) {
        xs[i] %= nowdiv;
      }
      sort(xs.begin(), xs.end() - 1);
    }
    // cout << endl;
  }
  cout << int(res / rev) << endl;
  return 0;
}
