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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int n;


string constr(const VI &seq) {
  int n = seq.size();
  string ret(n, '*');
  vector<char> list;
  REP(i, 0, 26) {
    list.push_back('a' + i);
  }
  REP(i, 0, n) {
    cerr << " " << seq[i];
  }
  cerr << endl;
  REP(i, 0, n) {
    int idx = seq[i];
    ret[i] = list[idx];
    list.erase(list.begin() + idx);
  }
  return ret;
}

#ifdef LOCAL
string dat;
void init(void) {
  if (0) {
    VI seq(n);
    mt19937 gen(time(0));
    REP(i, 0, n) {
      seq[i] = gen() % (26 - i);
    }
    dat = constr(seq);
  } else {
    cin >> dat;
  }
}

int ask(const VI &seq) {
  string s = constr(seq);
  if (s < dat) {
    return 1;
  }
  if (s > dat) {
    return -1;
  }
  exit(0);
}
#else
void init(void) {
}

int ask(const VI &seq) {
  string s = constr(seq);
  cout << s << endl;
  string res;
  cin >> res;
  if (res == "greater") {
    return 1;
  }
  if (res == "less") {
    return -1;
  }
  exit(0); // equal
}
#endif


VI halve(const VI &a, const VI &b) {
  int n = a.size();
  VI ret(n);
  int tmp = 0;
  for (int i = n - 1; i >= 0; --i) {
    int radix = 26 - i;
    tmp += a[i] + b[i];
    ret[i] = tmp % radix;
    tmp /= radix;
  }
  REP(i, 0, n) {
    int radix = 26 - i;
    tmp *= radix;
    tmp += ret[i];
    ret[i] = tmp / 2;
    tmp %= 2;
  }
  return ret;
}

int main(void) {
  cin >> n;
  init();
  VI lo(n), hi(n);
  hi[0] = 26;
  REP(_, 0, 90) {
    VI mid = halve(lo, hi);
    int res = ask(mid);
    if (res < 0) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
}
