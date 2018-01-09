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

ll inf = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll d;
  cin >> n >> d;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll cur = 0;
  ll curma = 0;
  ll cur_naive = 0;
  VL mi;
  VL ma;
  mi.push_back(-inf);
  REP(i, 0, n) {
    if (a[i] == 0) {
      mi.push_back(-cur);
      ma.push_back(d - curma);
      curma = cur;
      if (cur_naive < 0) {
	cur_naive = 0;
      }
    } else {
      cur += a[i];
      curma = max(curma, cur);
      cur_naive += a[i];
      if (cur_naive > d) {
	cout << -1 << "\n";
	return 0;
      }
    }
  }
  ma.push_back(d - curma);
  if (0) {
    REP(i, 0, mi.size()) {
      cerr << " " << mi[i];
    }
    cerr << endl;
    REP(i, 0, ma.size()) {
      cerr << " " << ma[i];
    }
    cerr << endl;
  }
  int cnt = 0;
  cur = 0;
  ll curmi = -inf;
  REP(i, 1, mi.size()) {
    if (curmi > ma[i]) {
      cur = -1;
      break;
    }
    if (mi[i] <= cur) {
      // x[i] can be 0, so x[i] = 0.
    } else {
      cnt += 1;
      cur = ma[i];
    }
    curmi = max(curmi, mi[i]);
  }
  cout << cnt << "\n";
}
