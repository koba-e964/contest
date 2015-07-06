#include <algorithm>
#include <cmath>
#include <complex>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;
typedef complex<double> comp;
int x[5], y[5];


struct cc {
  bool operator()(pair<double, comp> a, pair<double, comp> b) {
    return a.first < b.first;
  }
};

bool solve(comp a, comp p1, comp p2, comp p3) {
  double res = (conj(p3 - p1) * (p2 - p1)).imag();
  return res <= -EPS;
}

int main(void){
  double ax = 0, ay = 0;
  REP(i, 0, 5) {
    cin >> x[i] >> y[i];
    x[i] *= 5;
    y[i] *= 5;
    ax += x[i];
    ay += y[i];
  }
  ax /= 5;
  ay /= 5;
  vector<pair<double, comp> > rc;
  REP(i, 0, 5) {
    rc.push_back(pair<double, comp>(atan2(y[i] - ay, x[i] - ax), comp(x[i], y[i])));
  }
  sort(rc.begin(), rc.end(), cc());
  REP(i, 0, 5) {
    if (!solve(comp(ax, ay), rc[i].second, rc[(i+1)%5].second, rc[(i+2)%5].second)) {
      cout << "NO" << endl;
      return 0;
    }
  }
  cout << "YES" << endl;
}
