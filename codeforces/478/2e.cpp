#include <algorithm>
#include <cassert>
#include <cctype>
#include <complex>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iostream>
#include <vector>

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

typedef complex<long double> comp;

// Typical T: complex<double>, complex<long double>
// Reference: https://en.wikipedia.org/wiki/Centroid#Centroid_of_a_polygon
template<class T>
T centroid_polygon(const vector<T> &pts) {
  typedef typename T::value_type ty;
  int n = pts.size();
  ty area2 = 0; // 2 * area
  T cent = 0;
  T bias = pts[0];
  for (int i = 0; i < n; ++i) {
    ty outer = (conj(pts[i] - bias) * (pts[(i + 1) % n] - bias)).imag();
    area2 += outer;
    cent += ((pts[i] - bias) + (pts[(i + 1) % n] - bias)) * outer;
  }
  return cent / T(3 * area2, 0) + bias;
}


int main(void) {
  int n, q;
  scanf("%d%d", &n, &q);
  vector<comp> pt(n);
  REP(i, 0, n) {
    double x, y;
    scanf("%lf%lf", &x, &y);
    pt[i] = comp(x, y);
  }
  comp cent = centroid_polygon(pt);
  vector<comp> vec(n);
  REP(i, 0, n) vec[i] = pt[i] - cent;
  if (DEBUG) {
    cerr << "centroid = " << cent << endl;
  }
  vector<comp> argv(n);
  REP(i, 0, n) {
    argv[i] = vec[i] / abs(vec[i]);
  }
  int p1 = 0, p2 = 1;
  comp delta(1, 0);
  bool cache_alive = true;
  comp cache = pt[0];
  comp cur = cent; // The current position of the centroid
  REP(i, 0, q) {
    if (DEBUG) {
      DEBUGP(delta);
      DEBUGP(cur);
      cerr << "points:";
      REP(i, 0, n) {
	cerr << " " << cur + delta * vec[i];
      }
      cerr << endl;
    }
    int ty;
    scanf("%d", &ty);
    if (ty == 1) {
      int f, t;
      scanf("%d%d", &f, &t);
      f--, t--;
      if (p1 == f) {
	swap(p1, p2);
	if (p1 != p2)
	  cache_alive = false;
      }
      // What is the new value of cur?
      comp diff = delta * vec[p1];
      comp pinned = cache_alive ? cache : cur + diff;
      comp newdiff = comp(0, abs(vec[p1]));
      cur = pinned - newdiff;
      delta = comp(0, 1) * conj(argv[p1]);
      p2 = t;
      if (not cache_alive) {
	cache = pinned;
	cache_alive = true;
      }
    } else {
      int v;
      scanf("%d", &v);
      v--;
      comp ans = cur + delta * vec[v];
      printf("%.15f %.15f\n", (double) ans.real(), (double) ans.imag());
    }
  }
}
