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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;


bool ok(vector<double> &w, vector<double> &p, int k, double con) {
  int n = w.size();
  int cnt = 0;
  double tot = 0;
  double tot_nacl = 0;
  vector<pair<double, int> > contrib(n);
  REP(i, 0, n) {
    contrib[i] = pair<double, int>(-w[i] * (p[i] - con), i);
  }
  sort(contrib.begin(), contrib.end());
  double sum = 0;
  REP(i, 0, k) {
    sum -= contrib[i].first;
  }
  return sum >= 0;
}

int main(void){
  int n, k;
  cin >> n >> k;
  vector<double> w(n), p(n);
  REP(i, 0, n) {
    cin >> w[i] >> p[i];
  }
  double hi = 100;
  double lo = 0;
  REP(loop_cnt, 0, 50) {
    double mid = (hi + lo) / 2;
    if (ok(w, p, k, mid)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  printf("%.9f\n", hi);
}
