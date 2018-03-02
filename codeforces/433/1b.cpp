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
typedef pair<PI, int> PPII;

const ll inf = 2e12;

const int D=1001000;

ll cleft[D], cright[D];
vector<PPII> flights[D];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  REP(i, 0, m) {
    int d, f, t, c;
    cin >> d >> f >> t >> c;
    flights[d].push_back(PPII(PI(f, t), c));
  }
  VL cost(n + 1, inf);
  ll tot = n * inf;
  REP(d, 0, D) {
    REP(i, 0, flights[d].size()) {
      PPII ft = flights[d][i];
      int f = ft.first.first;
      int t = ft.first.second;
      int c = ft.second;
      if (t == 0) {
	ll oldc = cost[f];
	cost[f] = min(cost[f], (ll) c);
	tot += cost[f] - oldc;
      }
      if (0) {
	cerr << "cost:";
	for (auto v:cost)cerr<<" "<<v;
	cerr<<endl;
      }
    }
    cleft[d] = tot;
  }
  REP(i, 1, n + 1) cost[i] = inf;
  tot = n * inf;
  for (int d = D - 1; d >= 0; --d) {
    REP(i, 0, flights[d].size()) {
      PPII ft = flights[d][i];
      int f = ft.first.first;
      int t = ft.first.second;
      int c = ft.second;
      if (f == 0) {
	ll oldc = cost[t];
	cost[t] = min(cost[t], (ll) c);
	tot += cost[t] - oldc;
      }
    }
    cright[d] = tot;
  }
  ll mi = inf;
  REP(i, 0, D - k - 1) {
    mi = min(mi, cright[i + k + 1] + cleft[i]);
  }
  cout << (mi >= inf ? -1 : mi) << "\n";
}
