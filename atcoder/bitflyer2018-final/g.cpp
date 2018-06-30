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

const int DEBUG = 1;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

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

int n, m;
int taillen;
vector<VI> tails;
void dfs(int pos, VI &path) {
  if (pos >= taillen) {
    if (DEBUG) {
      REP(i, 0, path.size()) {
        cerr << " " << path[i];
      }
      cerr << endl;
    }
    tails.push_back(path);
    return;
  }
  int u = taillen - 1 - pos;
  REP(i, 0, u + 1) {
    path[u] = i;
    dfs(pos + 1, path);
  }
}

VI recover(const VI &path) {
  int len = path.size();
  VI tap(len);
  REP(i, 0, len) tap[i] = i;
  VI ret(len);
  for (int i = len - 1; i >= 0; --i) {
    ret[i] = tap[path[i]];
    tap.erase(tap.begin() + path[i]);
  }
  return ret;
}

ModInt<> solve_cons(const vector<PI> &cons, const vector<PI> &carrycons) {
  VI yuki(n, -1);
  ModInt<> tot(0);
  REP(carrypos, 0, n - taillen) {
    REP(last, 0, n - carrypos + 1) {
      bool uku = false;
      set<int> app;
      REP(i, 0, cons.size()) {
        int x = cons[i].first, y = cons[i].second;
        if (yuki[x] == -1) {
          yuki[x] = y;
        } else if (yuki[x] != y) {
          uku = true;
          break;
        }
        if (app.count(y)) {
          uku = true;
          break;
        }
        app.insert(y);
      }
      if (not uku) {
        set<int> rem;
        REP(i, 0, n) rem.insert(i);
        REP(i, 0, n - taillen) {
          if (yuki[i] != -1) {
            rem.erase(yuki[i]);
          }
        }
        VI tesla(rem.begin(), rem.end());
        sort(tesla.begin(), tesla.end());
        map<int, int> inv;
        REP(i, 0, tesla.size()) inv[tesla[i]] = i;
        vector<PI> poscons;
        REP(i, n - taillen, n) {
          if (yuki[i] != -1) {
            int idx = inv[yuki[i]];
            if (idx == -1) {uku = true; break; }
            poscons.push_back(PI(idx, i));
          }
        }
        poscons.insert(poscons.begin(), PI(-1, n - taillen - 1));
        poscons.push_back(PI(n, n));
        REP(i, 0, poscons.size() - 1) {
          if (poscons[i + 1].first - poscons[i].first < poscons[i + 1].second - poscons[i].second) {
            uku = true;
            break;
          }
        }
      }
      if (uku) continue;
      int cnt = 0;
      REP(i, 0, n - taillen) if (yuki[i] == -1) cnt++;
      if (DEBUG) {
        DEBUGP(cnt);
        REP(i, 0, yuki.size()) cerr << " " << yuki[i];
        cerr << endl;
        REP(i, 0, cons.size()) cerr << " (" << cons[i].first <<
          "," << cons[i].second << ")";
        cerr << endl;
      }
      ModInt<> tmp(1);
      REP(i, 1, cnt + 1) tmp = tmp * (i + 1);
      tot = tot + tmp;
    }
  }
  return tot;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m;
  VI a(m), b(m), c(m);
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> c[i];
    b[i]--;
    c[i]--;
  }
  int pos = 1;
  ll cnt = 1;
  while (pos < n && cnt <= 50) {
    cnt *= pos;
    pos++;
  }
  taillen = pos;
  DEBUGP(taillen);
  VI path(taillen);
  dfs(0, path);
  ModInt<> tot = 0;
  REP(i, 0, tails.size()) {
    vector<PI> cons;
    vector<PI> carrycons;
    REP(j, 0, m) {
      VI tail = tails[(i + a[j]) % tails.size()];
      int carry = i + a[j] >= (int) tails.size();
      VI act = recover(tail);
      if (DEBUG) {
        cerr << "tail = ";
        REP(i, 0, tail.size()) cerr << " " << tail[i];
        cerr << " act = ";
        REP(i, 0, act.size()) cerr << " " << act[i];
        cerr << endl;
      }
      vector<PI> &targ = carry ? carrycons : cons;
      if (n - b[j] - 1 <= taillen) {
        targ.push_back(PI(n - taillen + act[n - b[j] - 1], c[j]));
      } else {
        targ.push_back(PI(b[j], c[j]));
      }
    }
    tot = tot + solve_cons(cons, carrycons);
  }
  cout << tot << endl;
}
