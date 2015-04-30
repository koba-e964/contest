#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N = 100010;

double ax[N], ay[N], bx[N], by[N];

double solve(int n, double x[], double y[]) {
  double sq = 0, sumx = 0, sumy = 0;
  REP(i, 0, n) {
    sq += x[i] * x[i] + y[i] * y[i];
    sumx += x[i];
    sumy += y[i];
  }
  return sq - (sumx * sumx + sumy * sumy) / n;
}

int main(void){
  int n;
  cin >> n;
  REP(i,0,n) cin >> ax[i] >> ay[i];
  REP(i,0,n) cin >> bx[i] >> by[i];
  printf("%.10f\n", sqrt(solve(n,bx,by) / solve(n, ax, ay)));
}
