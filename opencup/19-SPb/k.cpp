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
const ll mod = 1e9 + 7;

ll gcd(ll a, ll b) {
  a = abs(a);
  b = abs(b);
  while (b != 0) {
    ll r = a % b;
    a = b; b = r;
  }
  return a;
}

struct quot {
  ll a, b;
  quot(ll a) : a(a), b(1) {}
  quot(ll a, ll b): a(a), b(b) {}
  quot(): a(0), b(1) {}
  quot operator+(quot o) const {
    ll num = a * o.b + b * o.a;
    ll den = b * o.b;
    ll g = gcd(num, den);
    num /= g;
    den /= g;
    return quot(num, den);
  }
  quot operator-(quot o) const {
    ll num = a * o.b - b * o.a;
    ll den = b * o.b;
    ll g = gcd(num, den);
    num /= g;
    den /= g;
    return quot(num, den);
  }
  quot operator*(quot o) const {
    ll num = a * o.a;
    ll den = b * o.b;
    ll g = gcd(num, den);
    num /= g;
    den /= g;
    return quot(num, den);
  }
  quot operator/(quot o) const {
    ll num = a * o.b;
    ll den = b * o.a;
    ll g = gcd(num, den);
    num /= g;
    den /= g;
    if (den < 0) {
      den = -den;
      num = -num;
    }
    return quot(num, den);
  }
  // TODO
  bool operator==(quot o) const {
    return a == o.a && b == o.b;
  }
  bool operator!=(quot o) const {
    return !(*this == o);
  }
  bool operator<(quot o) const {
    if (a != o.a) return a < o.a;
    return b < o.b;
  }
};

ostream& operator <<(ostream &os, quot q) {
  return os << q.a << "/" << q.b;
}

typedef vector<quot> vq;

// A: n * 4, b: n
int gauss_elim(vector<vq> &A, vq &b) {
  int n = A.size();
  int r = 0;
  REP(c, 0, 4) {
    if (r >= n) {
      break;
    }
    int r2 = -1;
    REP(i, r, n) {
      if (A[i][c] != 0) {
        r2 = i;
        break;
      }
    }
    if (r2 < 0) {
      continue;
    }
    swap(A[r], A[r2]);
    swap(b[r], b[r2]);
    quot f = A[r][c];
    REP(i, c, 4) {
      A[r][i] = A[r][i] / f;
    }
    b[r] = b[r] / f;
    REP(i, 0, n) {
      if (i == r) continue;
      quot g = A[i][c];
      REP(j, c, 4) {
        A[i][j] = A[i][j] - g * A[r][j];
      }
      b[i] = b[i] - g * b[r];
    }
    r++;
  }
  return r;
}

vector<vq> make(int n, int m, vector<quot> b) {
  vector<vq> ret(n, vq(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      quot tmp = b[0];
      tmp = tmp + b[1] * (i + 1) + b[2] * (j + 1);
      tmp = tmp + b[3] * (i + 1) * (j + 1);
      ret[i][j] = tmp;
    }
  }
  return ret;
}

void emit(const vector<vq> &A) {
  int n = A.size();
  int m = A[0].size();
  REP(i, 0, n) {
    REP(j, 0, m) {
      cout << A[i][j] << (j == m - 1 ? "\n" : " ");
    }
  }
}

void emit(int n, int m, vector<quot> b) {
  REP(i, 0, n) {
    REP(j, 0, m) {
      quot tmp = b[0];
      tmp = tmp + b[1] * (i + 1) + b[2] * (j + 1);
      tmp = tmp + b[3] * (i + 1) * (j + 1);
      cout << tmp << (j == m - 1 ? "\n" : " ");
    }
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<vector<quot> > a;
  vector<quot> b;
  REP(i, 0, n) {
    REP(j, 0, m) {
      string s;
      cin >> s;
      if (s != "?") {
        stringstream str(s);
        ll v;
        str >> v;
        vector<quot> t(4);
        t[0] = 1;
        t[1] = i + 1;
        t[2] = j + 1;
        t[3] = (i + 1) * (j + 1);
        a.push_back(t);
        b.push_back(v);
      }
    }
  }
  int sz = a.size();
  int rank = gauss_elim(a, b);
  if (DEBUG) {
    cerr << "A:" << endl;
    REP(i, 0, sz) {
      REP(j, 0, 4) cerr << " " << a[i][j];
      cerr << endl;
    }
    cerr << "b:" << endl;
    REP(i, 0, sz) cerr << " " << b[i];
    cerr << endl;
  }
  bool ok = 1;
  REP(i, rank, sz) {
    REP(j, 0, 4) {
      assert (a[i][j] == 0);
    }
    if (b[i] != 0) {
      ok = 0;
      break;
    }
  }
  if (not ok) {
    cout << "None" << endl;
    return 0;
  }
  set<vector<vq> > pool;
  vq nb(4);
  map<int, int> corr;
  set<int> det;
  REP(i, 0, rank) {
    int idx = -1;
    REP(j, 0, 4) {
      if (a[i][j] != 0) {
        idx = j;
        break;
      }
    }
    det.insert(idx);
    nb[idx] = b[i];
    corr[i] = idx;
  }
  pool.insert(make(n, m, nb));
  REP(j, 0, 4) {
    if (det.count(j)) continue;
    vq nb2(nb);
    nb2[j] = 1;
    REP(i, 0, sz) {
      int c = corr[i];
      nb2[c] = nb2[c] - a[i][j];
    }
    if(DEBUG) {
      cerr << "nb2:";
      REP(j, 0, 4) cerr << " " << nb2[j];
      cerr << endl;
    }
    pool.insert(make(n, m, nb2));
  }
  if (pool.size() == 1) {
    cout << "Unique" << endl;
    vector<vq> ans = *pool.begin();
    emit(ans);
    return 0;
  }
  cout << "Multiple" << endl;
  auto it = pool.begin();
  vector<vq> ans1 = *it;
  ++it;
  vector<vq> ans2 = *it;
  emit(ans1);
  cout << "and" << endl;
  emit(ans2);
}
