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
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
const double pi = acos(-1);

const int DEBUG = 1;

PL get(void) {
  ll x, y;
  cin >> x >> y;
  return PI(x, y);
}

const double inf = 5e15;
typedef pair<double, PI> PDPI;

double tertia(PL p1, PL p2, int n, vector<PL> &pt) {
  const double coef = 5 * pi - 20;
  // LIS, longest increasing subsequence
  VI m(n + 1);
  int l = 0;
  REP(i, 0, n) {
    int lo = 0;
    int hi = l + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (pt[m[mid]].second < pt[i].second) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    int newl = lo + 1;
    m[newl] = i;
    if (newl > l) {
      l = newl;
    }
  }
  int k = min(abs(p2.first - p1.first), abs(p2.second - p1.second)) + 1;
  double mi = 1.0 / 0.0;
  if (l == k) {
    mi = l * coef + 5.0 * pi;
  } else {
    mi = l * coef;
  }
  return mi;
}

double solve(PL p1, PL p2, int n, vector<PL> &pt) {
  // preprocess
  if (p1.first > p2.first) {
    p1.first = -p1.first;
    p2.first = -p2.first;
    REP(i, 0, n) {
      pt[i].first = -pt[i].first;
    }
  }
  if (p1.second > p2.second) {
    p1.second = -p1.second;
    p2.second = -p2.second;
    REP(i, 0, n) {
      pt[i].second = -pt[i].second;
    }
  }
  // trim
  vector<PL> pool;
  REP(i, 0, n) {
    if (p1.first <= pt[i].first && pt[i].first <= p2.first
	&& p1.second <= pt[i].second && pt[i].second <= p2.second) {
      pool.push_back(pt[i]);
    }
  }
  sort(pool.begin(), pool.end());
  int nn = pool.size();
  double mi = tertia(p1, p2, nn, pool);
  return mi;
}

int main(void) {
  PL p1 = get();
  PL p2 = get();
  int n;
  cin >> n;
  vector<PL> pt(n);
  REP(i, 0, n) {
    pt[i] = get();
  }
  double res = abs(p1.first - p2.first) + abs(p1.second - p2.second);
  res *= 100;
  double add = solve(p1, p2, n, pt);
  printf("%.15f\n", res + add);
}
