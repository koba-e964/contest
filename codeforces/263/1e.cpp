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

const int M = 100100;
ll hsh[M];

string en[4][4];
const ll inf = 4e18;

typedef vector<VL> mat;
mat mul(const mat &a, const mat &b) {
  mat ret(4, VL(4, inf));
  REP(i, 0, 4) {
    REP(j, 0, 4) {
      REP(k, 0, 4) {
        ret[i][k] = min(ret[i][k], a[i][j] + b[j][k]);
      }
    }
  }
  REP(i, 0, 4) REP(j, 0, 4) ret[i][j] = min(ret[i][j], inf);
  return ret;
}

mat pw(const mat &a, ll e) {
  mat ret(4, VL(4, inf));
  REP(i, 0, 4) ret[i][i] = 0;
  for (int i = 62; i >= 0; --i) {
    ret = mul(ret, ret);
    if (e & 1LL << i) {
      ret = mul(ret, a);
    }
  }
  return ret;
}

string yuki(int len, int pattern) {
  string s(len, '+');
  REP(i, 0, len) {
    int v = (pattern >> (2 * i)) & 3;
    s[len - i - 1] = 'A' + v;
  }
  return s;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  string t;
  cin >> t;
  int m = t.size();
  REP(i, 0, m) {
    hsh[i + 1] = 4 * hsh[i] + (t[i] - 'A');
  }
  {
    int dec = 0;
    int len = 2;
    while (dec < 16) {
      if (DEBUG) {
        DEBUGP(dec);
        DEBUGP(len);
      }
      vector<bool> s(1 << (2 * len));
      REP(i, 0, m - len + 1) {
        ll val = hsh[i + len] - (hsh[i] << (2 * len));
        s[val] = true;
      }
      // TODO optimise
      REP(i, 0, 1 << (2 * len)) {
        if (not s[i]) {
          int from = i % 4;
          int to = i >> (2 * (len - 1));
          if (en[from][to] == "") {
            en[from][to] = yuki(len, i);
            dec++;
          }
        }
      }
      len++;
    }
    if (DEBUG) {
      REP(i, 0, 4) {
        DEBUGP(i);
        REP(j, 0, 4) {
          DEBUGP(en[i][j]);
        }
      }
    }
  }
  int minlen = 100;
  REP(i, 0, 4) REP(j, 0, 4) minlen = min(minlen, (int) en[i][j].size());
  mat irnbru(4, VL(4));
  REP(i, 0, 4) REP(j, 0, 4) irnbru[i][j] = en[i][j].size() - 1;
  ll pass = 0, fail = (n + minlen - 2) / (minlen - 1);
  while (fail - pass > 1) {
    ll mid = (fail + pass) / 2;
    mat qnighy = pw(irnbru, mid);
    ll mi = inf;
    REP(i, 0, 4) REP(j, 0, 4) mi = min(mi, qnighy[i][j]);
    if (mi <= n - 1) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  if (DEBUG) {
    REP(i, max(0LL, fail - 4), fail + 2) {
      mat takoyaki = pw(irnbru, i);
      cerr << "irnbru^" << i << ":\n";
      REP(i, 0, 4) {
        REP(j, 0, 4) cerr << " " << takoyaki[i][j];
        cerr << endl;
      }
    }
  }
  cout << fail << "\n";
}
