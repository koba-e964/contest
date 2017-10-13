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

bool check(const string &s, double r, int k) {
  int n = s.length();
  vector<double> tbl(2 * n);
  tbl[0] = 0;
  REP(i, 1, 2 * n) {
    tbl[i] = tbl[i - 1] - r + (s[(i - 1) % n] == '1' ? 1 : 0);
  }
  double mi = 0;
  REP(i, k, 2 * n) {
    mi = min(mi, tbl[i - k]);
    if (mi <= tbl[i]) {
      return true;
    }
  }
  return false;
}


int main(void) {
  int n, k;
  cin >> n >> k;
  string s;
  cin >> s;
  double lo = 0, hi = 1;
  REP(_, 0, 60) {
    double mid = (lo + hi) / 2;
    if (check(s, mid, k)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  printf("%.15f\n", lo);
}
