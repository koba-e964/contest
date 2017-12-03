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
const ll mod = 1e9 + 7;


const int N = 1010;
int n, m, c;

#ifdef LOCAL
int loc[N];
int last_ask;
int ask(void) {
  last_ask = 1;
  return last_ask;
}
void output(int v) {
  loc[v] = last_ask;
}
#else
int ask(void) {
  int p;
  cin >> p;
  return p;
}
void output(int v) {
  cout << v + 1 << endl;
}
#endif



int main(void) {
  cin >> n >> m >> c;
  VI a(n, -1);
  int lim = (c + 1) / 2;
  // [1,lim]: first, [lim + 1, c]: last
  REP(i, 0, m) {
    int p = ask();
    int ind = -1;
    if (p <= lim) {
      REP(i, 0, n) {
	if (p < a[i] || a[i] == -1) {
	  ind = i;
	  break;
	}
      }
    } else {
      for (int i = n - 1; i >= 0; --i) {
	if (p > a[i] || a[i] == -1) {
	  ind = i;
	  break;
	}
      }
    }
    assert (ind >= 0);
    output(ind);
    a[ind] = p;
    bool ok = true;
    REP(i, 0, n - 1) {
      if(a[i] == -1 || a[i + 1] == -1 || a[i] > a[i + 1]) {
	ok = false;
	break;
      }
    }
    if (ok) {
      return 0;
    }
    if (0) {
      cerr << "a:";
      for(auto v:a)cerr << " " << v;
      cerr << endl;
    }
  }
}
