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

#define MOCK 0

#if MOCK
mt19937 mt;
string s, t;
void init(void) {
  int n = 100;
  s = string(n, 'a'), t = string(n, 'a');
  REP(i, 0, n) {
    s[i] = mt() % 2 ? '1' : '0';
    t[i] = mt() % 2 ? '1' : '0';
  }
}
string ask(void) {
  string r = mt() % 2 ? s : t;
  int n = 100;
  VI a(n);
  REP(i, 0, 100) a[i] = i;
  REP(i, 0, 100) {
    int j = mt() % (i + 1);
    swap(a[i], a[j]);
  }
  REP(i, 0, 15) {
    r[a[i]] = '0' + '1' - r[a[i]];
  }
  return r;
}
void sure(string ss, string tt) {
  if (ss > tt) swap(ss, tt);
  if (s == ss && t == tt) {
    cout << "ok" << endl;
  } else {
    cout << "err" << endl;
    DEBUGP(s);
    DEBUGP(t);
    DEBUGP(ss);
    DEBUGP(tt);
  }
}

#else
void init(void) {}
string ask(void) {
  cout << "?" << endl;
  string s;
  cin >> s;
  return s;
}

void sure(string s, string t) {
  if (s > t) swap(s, t);
  cout << "! " << s << " " << t << endl;
}
#endif

int main(void) {
  init();
  int n, k, q;
#if MOCK
  n = 100;
  k = 15;
  q = 100;
#else
  cin >> n >> k >> q;
#endif
  vector<string> r(q);
  REP(i, 0, q) {
    r[i] = ask();
  }
  string uni(n, '+');
  vector<bool> dif(n);
  VI didx;
  REP(i, 0, n) {
    int zero = 0;
    int one = 0;
    REP(j, 0, q) {
      if (r[j][i] == '1') one++;
      else zero++;
    }
    if (min(zero, one) <= 32) {
      dif[i] = false;
      uni[i] = zero < one ? '1' : '0';
    } else {
      dif[i] = true;
      didx.push_back(i);
    }
  }
  if (didx.size() == 0) {
    sure(uni, uni);
    return 0;
  }

  vector<bool> a(n), b(n);
  REP(i, 0, n) {
    a[i] = b[i] = uni[i] == '1' ? true : false;
  }
  int piv = didx[0];
  b[piv] = true;
  REP(i, 1, didx.size()) {
    int v = didx[i];
    // I want s[piv] xor s[v]. This value is the same for t.
    int zero = 0;
    int one = 0;
    REP(j, 0, q) {
      if (r[j][piv] != r[j][v]) one++;
      else zero++;
    }
    if (zero > one) {
      a[v] = false;
      b[v] = true;
    } else {
      a[v] = true;
      b[v] = false;
    }
  }
  string s(n, '+'), t(n, '+');
  REP(i, 0, n) {
    s[i] = a[i] ? '1' : '0';
    t[i] = b[i] ? '1' : '0';
  }
  sure(s, t);
}
