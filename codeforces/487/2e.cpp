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
#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y; y = r;
  }
  return x;
}

struct line {
  // Denotes ax + by = c
  ll a, b, c;
  line(ll a, ll b, ll c): a(a), b(b), c(c) {}
  static line cross(PL x, PL y) {
    ll a = y.second - x.second;
    ll b = x.first - y.first;
    ll c = a * x.first + b * x.second;
    ll g = a;
    g = gcd(g, b);
    g = gcd(g, c);
    a /= g;
    b /= g;
    c /= g;
    if (b < 0) {
      a = -a;
      b = -b;
      c = -c;
    }
    if (b == 0 && a < 0) {
      a = -a;
      c = -c;
    }
    return line(a, b, c);
  }
  bool operator<(const line &other) const {
    if (a != other.a) return a < other.a;
    return PL(b, c) < PL(other.b, other.c);
  }
  bool operator==(const line &other) const {
    return a == other.a && b == other.b && c == other.c;
  }
  bool inhabit(PL p) const {
    return a * p.first + b * p.second == c;
  }
};

ostream &operator<<(ostream &os, const line &l) {
  return os << l.a << "x + " << l.b << "y = " << l.c << endl;
}


typedef vector<vector<double> > mat;

mat mul(const mat &a, const mat &b) {
  int n = a.size();
  int m = b.size();
  int l = b[0].size();
  mat ret(n, vector<double>(l, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
        ret[i][k] += a[i][j] * b[j][k];
      }
    }
  }
  return ret;
}

int main(void) {
  int n;
  scanf("%d", &n);
  VI x(n), y(n);
#if MOCK
  mt19937 mt;
  REP(i, 0, n) {
    x[i] = mt() % 10000, y[i] = mt() % 10000;
  }
#else
  REP(i, 0, n) scanf("%d%d", &x[i], &y[i]);
#endif
  vector<line> sigma;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      sigma.push_back(line::cross(PL(x[i], y[i]), PL(x[j], y[j])));
    }
  }
  sort(sigma.begin(), sigma.end());
  sigma.erase(unique(sigma.begin(), sigma.end()), sigma.end());
  if (DEBUG) {
    REP(i, 0, sigma.size()) {
      cerr << sigma[i];
    }
  }
  int e = sigma.size();
  mat a(n, vector<double>(n));
  mat b(n, vector<double>(e));
  mat c(e, vector<double>(n));
  REP(i, 0, n) {
    int sum = 0;
    REP(j, 0, e) {
      if (sigma[j].inhabit(PL(x[i], y[i]))) {
        sum += 1;
      }
    }
    REP(j, 0, e) {
      if (sigma[j].inhabit(PL(x[i], y[i]))) {
        b[i][j] = 1.0 / sum;
      }
    }
  }
  REP(j, 0, e) {
    int u = 0;
    REP(k, 0, n) {
      if (sigma[j].inhabit(PL(x[k], y[k]))) {
        u++;
      }
    }
    REP(k, 0, n) {
      if (sigma[j].inhabit(PL(x[k], y[k]))) {
        c[j][k] += 1.0 / u;
      }
    }
  }
  // TODO O(n^4)
  REP(i, 0, n) {
    REP(j, 0, e) {
      REP(k, 0, n) {
        a[i][k] += b[i][j] * c[j][k];
      }
    }
  }
  if (DEBUG) {
    cerr << "a:" << endl;
    REP(i, 0, n) {
      REP(j, 0, n) {
        cerr << " " << a[i][j];
      }
      cerr << endl;
    }
    cerr << "b:" << endl;
    REP(i, 0, n) {
      REP(j, 0, e) {
        cerr << " " << b[i][j];
      }
      cerr << endl;
    }
    cerr << "c:" << endl;
    REP(i, 0, e) {
      REP(j, 0, n) {
        cerr << " " << c[i][j];
      }
      cerr << endl;
    }
    mat uu(a);
    REP(i, 0, 30) {
      cerr << "a^" << (i + 1) << ":" << endl;
      REP(i, 0, n) {
        REP(j, 0, n) {
          cerr << " " << uu[i][j];
        }
        cerr << endl;
      }
      uu = mul(uu, a);
    }
  }
  // Okay, let's assume a^THR is idempotent.
  int THR = 30;
  vector<mat> chino(THR, mat(n, vector<double>(n, 0)));
  REP(i, 0, n) {
    chino[0][i][i] = 1;
  }
  REP(i, 1, THR) {
    chino[i] = mul(chino[i - 1], a);
  }
  int q;
  scanf("%d", &q);
  REP(i, 0, q) {
    int t, m;
    scanf("%d%d", &t, &m);
    t--;
    m = min(m, THR - 1);
    double ans = 0.0;
    REP(i, 0, n) {
      ans = max(ans, chino[m][i][t]);
    }
    REP(i, 0, e) {
      double sum = 0;
      REP(j, 0, n) {
        sum += chino[m - 1][j][t] * c[i][j];
      }
      ans = max(ans, sum);
    }
    // TODO
    printf("%.15f\n", ans);
  }
  
}
