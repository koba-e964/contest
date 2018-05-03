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
typedef pair<ll, ll> PL;


// Calculates the number of collisions.
ll calc(const vector<PL> &occ) {
  int n = occ.size();
  map<ll, ll> tbl;
  REP(i, 0, n) {
    tbl[occ[i].second]++;
  }
  ll tot = (ll)n * (ll)(n - 1) / 2;
  for (auto &e: tbl) {
    tot -= e.second * (e.second - 1) / 2;
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll a, b;
  cin >> n >> a >> b;
  VL x(n), vx(n), vy(n);
  REP(i, 0, n) {
    cin >> x[i] >> vx[i] >> vy[i];
    vy[i] -= a * vx[i];
  }
  map<ll, vector<PL> > tbl;
  REP(i, 0, n) tbl[vy[i]].push_back(PL(x[i], vx[i]));
  ll tot = 0;
  for (auto &e: tbl) {
    vector<PL> &occ = e.second;
    sort(occ.begin(), occ.end());
    tot += calc(occ);
  }
  cout << 2 * tot << endl;
}
