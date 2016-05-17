#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 105000;
const int M = 1e6;

int main(void){
  ll l, h;
  cin >> l >> h;
  VI a(N);
  REP(i, 2, N) a[i] = 1;
  REP(i, 2, N) {
    REP(j, 2, N / i) {
      a[i * j] = 0;
    }
  }
  // a: prime
  ll ma = 0;
  ll maxv = 0;
  for (ll p = N - 1; p >= max(ma, 2LL); --p) {
    if (a[p] == 0) {
      continue;
    }
    for (ll i = h / p; i >= p && i * p >= l; --i) {
      ll minfact = 2;
      for(; minfact < p; ++minfact) {
	if (i % minfact == 0) {
	  break;
	}
      }
      assert (a[minfact]);
      if (minfact >= ma) {
	maxv = minfact == ma ? max(maxv, i * p) : i * p;
	ma = minfact;
      }
      if (minfact == p) {
	break;
      }
    }
  }
  cout << maxv << endl;
  return 0;
}
