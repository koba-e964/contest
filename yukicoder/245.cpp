#include <iostream>
#include <complex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef complex<double> comp;
const double EPS=1e-9;

const int N = 100;

double a[N], b[N], c[N], d[N];
comp st[N], en[N];
comp pool[2 * N];
bool cross(comp x, comp y, comp z, comp w) {
  double ex1 = (conj(w - x) * (y - x)).imag();
  double ex2 = (conj(z - x) * (y - x)).imag();
  return ex1 * ex2 < EPS; // <= 0
}

int main(void) {
  int n;
  cin >> n;
  REP(i, 0, n) {
    double a, b, c, d;
    cin >> a >> b >> c >> d;
    pool[2 * i] = st[i] = comp(a, b);
    pool[2 * i + 1] = en[i] = comp(c, d);
  }
  int ma = 0;
  REP(i, 0, 2 * n) {
    REP(j, i + 1, 2 * n) {
      int c = 0;
      if (abs(pool[i] - pool[j]) < EPS) {
	continue;
      }
      REP(k, 0, n) {
	if (cross(pool[i], pool[j], st[k], en[k])) {
	  c++;
	}
      }
      ma = max(ma, c);
    }
  }
  cout << ma << endl;
}
