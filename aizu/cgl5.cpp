#include <algorithm>
#include <cmath>
#include <vector>

/*
 * Closest pair.
 */
class ClosestPair {
private:
  std::vector<double> x, y;
  struct x_cmp {
    const ClosestPair &cp;
    x_cmp(const ClosestPair &cp) : cp(cp) {} 
    bool operator() (int i, int j) {
      return cp.x[i] < cp.x[j];
    }
  };
  struct y_cmp {
    const ClosestPair &cp;
    y_cmp(const ClosestPair &cp) : cp(cp) {} 
    bool operator() (int i, int j) {
      return cp.y[i] < cp.y[j];
    }
  };
  x_cmp xc;
  y_cmp yc;
  double gl_mi; // global minimum;
  double dist2(int i, int j) const {
    return pow(x[i] - x[j], 2) + pow(y[i] - y[j], 2);
  }

  void min_brute_force(const std::vector<int> & v) {
    int n = v.size();
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
	if (i == j) {
	  continue;
	}
	gl_mi = std::min(gl_mi, dist2(v[i], v[j]));
      }
    }
  }


  void min_strip(const std::vector<int> &v) {
    int n = v.size();
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n && pow(y[v[j]] - y[v[i]], 2) < gl_mi; ++j) {
	gl_mi = std::min(gl_mi, dist2(v[i], v[j]));
      }
    }
  }

  /*
   */
  void min_pair_util(const std::vector<int> &vx) {
    int n = vx.size();
    if (n <= 4) {
      min_brute_force(vx);
      return;
    }
    // divide at middle
    int mid = n / 2;
    double mid_x = x[vx[mid]];
    min_pair_util(std::vector<int>(vx.begin(), vx.begin() + mid));
    min_pair_util(std::vector<int>(vx.begin() + mid, vx.end()));
    std::vector<int> strip;
    for(int i = 0; i < n; ++i) {
      if (pow(x[vx[i]] - mid_x, 2) < gl_mi) {
	strip.push_back(vx[i]);
      }
    }
    std::sort(strip.begin(), strip.end(), yc);
    min_strip(strip);
  }
  ClosestPair(const std::vector<double> &xs, const std::vector<double> &ys) : x(xs), y(ys), xc(*this), yc(*this), gl_mi(1.0 / 0.0) {
  }
  double internal_solve(void) {
    int n = x.size();
    std::vector<int> v(n);
    for (int i = 0; i < n; ++i) {
      v[i] = i;
    }
    std::sort(v.begin(), v.end(), xc);
    min_pair_util(v);
    return std::sqrt(gl_mi);
  }
public:
  static double solve(const std::vector<double> &xs, const std::vector<double> &ys) {
    return ClosestPair(xs, ys).internal_solve();    
  }
};
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

#include <cstdio>
#include <iostream>

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

int main(void){
  int n;
  cin >> n;
  vector<double> x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  printf("%.9f\n", ClosestPair::solve(x, y));
}
