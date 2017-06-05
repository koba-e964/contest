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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 1234;
int n;
double x[N], y[N];


double sq(double x) {
  return x * x;
}

double check(int u, int v, int w) {
  double dist = sqrt(sq(x[u] - x[w]) + sq(y[u] - y[w]));
  double outer = (x[w] - x[u]) * (y[v] - y[u]) - (y[w] - y[u]) * (x[v] - x[u]);
  return outer / dist / 2;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  double mi = 1.0 / 0.0;
  REP(i, 0, n) {
    mi = min(mi, check(i, (i + 1) % n, (i + 2) % n));
  }
  printf("%.15f\n", mi);
}
