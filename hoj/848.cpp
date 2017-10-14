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
typedef pair<ll, int> PLI;

const int inf = 1e8;

int main(void) {
  int n;
  ll c;
  cin >> n >> c;
  VL a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  vector<PLI> pool;
  REP(i, 0, n) {
    cin >> b[i];
    pool.push_back(PLI(b[i], i));
  }
  sort(pool.begin(), pool.end());
  pair<ll, PI> mi(5e15, PI(inf, inf));
  REP(i, 0, n) {
    if (a[i] == 0) {
      mi = min(mi, make_pair(c, PI(i, 0)));
    } else {
      ll tmp = (c + a[i] - 1) / a[i];
      vector<PLI>::iterator it = lower_bound(pool.begin(), pool.end(),
					     PLI(tmp, -1));
      if (it != pool.begin()) {
	ll v = (it - 1)->first;
	int idx = (it - 1)->second;
	mi = min(mi, make_pair(abs(c - a[i] * v), PI(i, idx)));
      }
      if (it != pool.end()) {
	ll v = it->first;
	int idx = it->second;
	mi = min(mi, make_pair(abs(c - a[i] * v), PI(i, idx)));
      }
    }
  }
  cout << mi.second.first + 1 << " " << mi.second.second + 1 << endl;
}
