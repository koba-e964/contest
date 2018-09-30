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
int x;
void init(void) {
  mt19937 mt(time(0));
  cin >> x;
  // x = mt() % (ll)1e9;
}
int ask(int q) {
  return (x % q) % 2;
}
bool sure(int myx) {
  // cerr << "x = " << x << endl;
  if (x != myx) {
    cerr << "Error! x = " << x << " given = " << myx << endl;
    return false;
  }
  return true;
}
#else
void init(void){}
int ask(int q) {
  if (q > (ll) 1e9) {
    q = 2;
  }
  cout << "? " << q << endl;
  string r;
  cin >> r;
  return r == "odd" ? 1 : 0;
}
void sure(int myx) {
  cout << "! " << myx << endl;
  exit(0);
}
#endif

#if MOCK
int solve(int tx) {
  x = tx;
  int o = ask(2);
  ll div = 1;
  for (int i = 29; i >= 1; --i) {
    int q = div + (1 << i);
    int u = ask(q);
    if (o != u) {
      div += 1 << i;
    }
  }
  return sure(div + 1 - o);
}

int main(void) {
  mt19937 mt(time(0));
  REP(i, 1, (ll)1e9 + 1) {
    if (i % 10000000 == 0) {
      cerr << i << endl;
    }
    int x = i;
    solve(x);
  }
}
#else

int main(void) {
  init();
  int o = ask(2);
  ll div = 1;
  for (int i = 29; i >= 1; --i) {
    int q = div + (1 << i);
    int u = ask(q);
    if (o != u) {
      div += 1 << i;
    }
  }
  sure(div + 1 - o);
}
#endif
