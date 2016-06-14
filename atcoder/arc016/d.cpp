#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;


const int N = 101;
const int H = 101;
const int DEBUG = 0;

int n, h;
vector<int> edges[N];
int d[N];
double dp[N][H];

double rec(int v, int hp, double e) {
  double &ret = dp[v][hp];
  assert (hp > 0);
  if (ret >= 0) {
    return ret;
  }
  if (v == n - 1) {
    return ret = 0;
  }

  int ne = edges[v].size();

  if (ne == 0) {
    return e + h - hp; // dead end
  }

  double sum = 0;
  REP(i, 0, ne) {
    if (hp <= d[edges[v][i]]) {
      sum = 1.0 / 0.0; // impossible!
    } else {
      sum += rec(edges[v][i], hp - d[edges[v][i]], e);
    }
  }
  ret = min(e + h - hp, sum / ne + 1);
  if (DEBUG) {
    cerr << "rec(" << v << "," << hp << "," << e << ")=" << ret << endl;
  }
  return ret;
}


bool ok(double e) {
  REP(i, 0, n) {
    REP(j, 0, h + 1) {
      dp[i][j] = -1.0;
    }
  }
  double r = rec(0, h, e);
  return r + EPS < e;
}

int main(void){
  int m;
  cin >> n >> m >> h;
  
  REP(i, 0, m) {
    int f, t;
    cin >> f >> t;
    f--, t--;
    edges[f].push_back(t);
  }
  REP(i, 0, n) {
    cin >> d[i];
  }

  double lo = 0.0;
  double hi = 1e8;
  REP(bin_cnt, 0, 100) {
    double mid = (lo + hi) / 2;
    bool o = ok(mid);
   if (o) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  printf("%.10f\n", lo >= 1e6 ? -1 : lo);
}
