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
typedef pair<double, double> PD;
typedef pair<double, PD> PDPD;

const int N = 100100;
const double EPS = 1e-9;
double x[N], y[N];
string d[N];

PD pre(const vector<double> &p) {
  double ma = -1.0 / 0;
  double mi = 1.0 / 0;
  for (auto pp: p) {
    ma = max(ma, pp);
    mi = min(mi, pp);
  }
  return PD(mi, ma);
}

vector<PDPD> calc(const vector<double> &pos,
                const vector<double> &neg,
                const vector<double> &stat) {
  PD pa = pre(pos), na = pre(neg), sa = pre(stat);
  vector<PD> mp;
  if (pa.first <= pa.second) {
    mp.push_back(PD(pa.first, 1));
    mp.push_back(PD(pa.second, 1));
  }
  if (na.first <= na.second) {
    mp.push_back(PD(na.first, -1));
    mp.push_back(PD(na.second, -1));
  }
  if (sa.first <= sa.second) {
    mp.push_back(PD(sa.first, 0));
    mp.push_back(PD(sa.second, 0));
  }
  if (DEBUG) {
    REP(i, 0, mp.size()) {
      cerr << "line:" << mp[i].first << " " << mp[i].second << endl;
    }
  }
  vector<double> event;
  event.push_back(0.0);
  int m = mp.size();
  REP(i, 0, m) {
    REP(j, 0, i) {
      if (mp[i].second != mp[j].second) {
        double col = -(mp[i].first - mp[j].first) / (mp[i].second - mp[j].second);
        if (col >= 0.0) {
          event.push_back(col);
        }
      }
    }
  }
  sort(event.begin(), event.end());
  vector<PDPD> ret;
  for (double t: event) {
    vector<PD> u;
    for (PD l: mp) {
      l.first += l.second * t;
      u.push_back(l);
    }
    sort(u.begin(), u.end());
    double grad_diff = u.back().second - u[0].second;
    double dist = u.back().first - u[0].first;
    ret.push_back(PDPD(t, PD(dist, grad_diff)));
  }
  return ret;
}



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> x[i] >> y[i] >> d[i];
  }
  vector<vector<PDPD> > gr;
  REP(_, 0, 2) {
    vector<double> pos, neg, stat;
    REP(i, 0, n) {
      if (d[i] == "R") {
        pos.push_back(x[i]);
      } else if (d[i] == "L") {
        neg.push_back(x[i]);
      } else {
        stat.push_back(x[i]);
      }
    }
    gr.push_back(calc(pos, neg, stat));
    gr.back().push_back(PDPD(1.0 / 0, PD(-1, -1)));
    // flip
    REP(i, 0, n) {
      swap(x[i], y[i]);
      string to = "U";
      if (d[i] == "U") {
        to = "R";
      } else if (d[i] == "D") {
        to = "L";
      }
      d[i] = to;
    }
    if (DEBUG) {
      for (auto e: gr.back()) {
        cerr << e.first << " " << e.second.first << " + " << e.second.second <<
          "(x - t)" << endl;
      }
    }
  }
  double mi = 1.0 / 0;
  REP(i, 0, gr[0].size() - 1) {
    REP(j, 0, gr[1].size() - 1) {
      // intersect?
      double lo = max(gr[0][i].first, gr[1][j].first);
      double hi = min(gr[0][i + 1].first, gr[1][j + 1].first);
      if (lo > hi * (1 + EPS)) continue;
      double dur = hi - lo;
      PD former(gr[0][i].second), latter(gr[1][j].second);
      former.first += (lo - gr[0][i].first) * former.second;
      latter.first += (lo - gr[1][j].first) * latter.second;
      double c2 = former.second * latter.second;
      double c1 = former.second * latter.first + former.first * latter.second;
      double c0 = former.first * latter.first;
      if (c2 == 0) {
        mi = min(mi, c0);
        mi = min(mi, c0 + dur * c1);
        continue;
      }
      double mid = -c1 / 2 / c2;
      if (0 <= mid && mid < dur) {
        double tmp = -c1 * c1 / 4 / c2 + c0;
        mi = min(mi, tmp);
      }
      mi = min(mi, c0);
      mi = min(mi, c0 + dur * (c1 + dur * c2));
    }
  }
  cout << fixed << setprecision(15) << mi << endl;
}
