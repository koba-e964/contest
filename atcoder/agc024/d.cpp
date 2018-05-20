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
typedef pair<int, ll> PIL;

const int N = 110;
VI g[N];

void chmax_vi(VI &x, const VI &y) {
  while (x.size() < y.size()) x.push_back(0);
  REP(i, 0, y.size()) {
    x[i] = max(x[i], y[i]);
  }
}


VI flatten(int v, int par) {
  VI ma;
  REP(i, 0, g[v].size()) {
    int w = g[v][i];
    if (w == par) continue;
    VI res = flatten(w, v);
    chmax_vi(ma, res);
  }
  ma.insert(ma.begin(), g[v].size() - 1);
  return ma;
}



const ll inf = 1e18;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    g[x].push_back(y);
    g[y].push_back(x);
  }
  // pivot brute force
  PIL mi(1e8, -1);
  REP(i, 0, n) {
    VI ma;
    REP(j, 0, g[i].size()) {
      int w = g[i][j];
      VI fl = flatten(w, i);
      if (DEBUG) {
	cerr << "flatten(" << w << " " << i << "):" << endl;
	REP(i, 0, fl.size()) {
	  cerr << " " << fl[i];
	}
	cerr << endl;
      }
      while (ma.size() < fl.size()) ma.push_back(0);
      REP(i, 0, fl.size()) ma[i] = max(ma[i], fl[i]);
    }
    ll prod = g[i].size();
    REP(j, 0, ma.size() - 1) {
      ll lim = inf / ma[j] + 1;
      if (prod >= lim) {
	prod = inf;
      } else {
	prod *= ma[j];
      }
    }
    mi = min(mi, PIL(ma.size() + 1, prod));
  }
  // TODO edge-centric
  REP(i, 0, n) {
    REP(j, 0, g[i].size()) {
      int w = g[i][j];
      if (i > w) continue;
      VI f1 = flatten(w, i);
      VI f2 = flatten(i, w);
      if (DEBUG) {
	DEBUGP(i);
	DEBUGP(w);
	cerr << "f1:" << endl;
	REP(i, 0, f1.size()) cerr << " " << f1[i];
	cerr << endl;
	cerr << "f2:" << endl;
	REP(i, 0, f2.size()) cerr << " " << f2[i];
	cerr << endl;
      }
      chmax_vi(f1, f2);
      VI ma(f1);
      ll prod = 2;
      REP(j, 0, ma.size() - 1) {
	ll lim = inf / ma[j] + 1;
	if (prod >= lim) {
	  prod = inf;
	} else {
	  prod *= ma[j];
	}
      }
      mi = min(mi, PIL(ma.size(), prod));
    }
  }
  cout << mi.first << " " << mi.second << endl;
}
