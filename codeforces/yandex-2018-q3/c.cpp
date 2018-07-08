#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <set>

const int DEBUG = 0;

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

void coord(VL &x) {
  sort(x.begin(), x.end());
  x.erase(unique(x.begin(), x.end()), x.end());
}

// Points (x, y) that satisfy x - h < y <= x
ll solve(const vector<PL> &pts, ll h) {
  int n = pts.size();
  if (DEBUG) {
    cerr << "pts:";
    REP(i, 0, n) {
      cerr << " (" << pts[i].first << "," << pts[i].second << ")";
    }
    cerr << endl;
  }
  VL rows, cols;
  REP(i, 0, n) {
    rows.push_back(pts[i].first);
    cols.push_back(pts[i].second);
  }
  coord(rows);
  coord(cols);
  ll tot = ((ll) rows.size() + (ll) cols.size()) * h;
  // How many intersections?
  for (ll row: rows) {
    int idx1 = upper_bound(cols.begin(), cols.end(), row - h) - cols.begin();
    int idx2 = upper_bound(cols.begin(), cols.end(), row) - cols.begin();
    tot -= idx2 - idx1;
  }
  
  // Cells occupied by Bishops don't count
  tot -= n;
  return tot;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll h;
  cin >> n >> h;
  VL x(n), y(n);
  REP(i, 0, n) cin >> x[i] >> y[i];
  vector<PL> even, odd;
  REP(i, 0, n) {
    ll z = x[i] + y[i];
    ll w = x[i] - y[i];
    if (z % 2 == 0) {
      even.push_back(PL(z / 2, w / 2));
    } else {
      odd.push_back(PL((z - 1) / 2, (w - 1) / 2));
    }
  }
  cout << solve(odd, h) + solve(even, h) << endl;
}
