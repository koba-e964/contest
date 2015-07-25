#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>
#include <complex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
typedef complex<double> comp;
const double EPS=1e-9;

const int N = 11;
comp vert[N];

double dist(comp b, comp c) {
  double k1 = norm(b);
  double k3 = norm(c);
  double k2 = (b * conj(c)).real();
  double w1 = k1 + k3 - 2 * k2;
  double w2 = k3 - k2;
  double w3 = k3;
  if (w1 < EPS || w3 < EPS) {
    return min(abs(b), abs(c));
  }
  if (w2 >= 0 && w2 <= w1) {
    double m = w2 / w1;
    double res = w1 * m * m - 2 * w2 * m + w3;
    return sqrt(res);
  }
  return min(abs(b), abs(c));  
}

int main(void){
  double x, y;
  int n;
  cin >> x >> y >> n;
  REP(i, 0, n) {
    double a, b;
    cin >> a >> b;
    vert[i] = comp(a, b);
  }
  double mi = 1.0 / 0.0;
  REP(i, 0, n) {
    comp q = comp(x, y);
    mi = min(mi, dist(vert[i] - q, vert[(i + 1) % n] - q));
  }
  printf("%.9f\n", mi);
}
