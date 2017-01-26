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

typedef pair<double, int> PDI;
const int N = 30;

vector<PDI> edges[N];



void rec(int v, vector<double> &pool, double p = 1.0) {
  if (v == 0) {
    pool.push_back(p);
    return;
  }
  for (auto w: edges[v]) {
    rec(w.second, pool, p * w.first);
  }
}

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int a, b;
    double c;
    cin >> a >> b >> c;
    c /= 100;
    edges[b].push_back(PDI(c, a));
  }
  double sum = 0.0;
  REP(bits, 1 << (n - 1), 1 << n) {
    double prod = 1;
    if ((bits & 1) == 0) { continue; }
    REP(i, 1, n) {
      double tmp = 1.0; // P(X_i = false | X_w = ... (w in edges[i])) 
      for (auto w: edges[i]) {
	if (bits & 1 << w.second) {
	  tmp *= 1 - w.first;
	}
      }
      if (bits & 1 << i) {
	prod *= 1 - tmp;
      } else {
	prod *= tmp;
      }
    }
    sum += prod;
  }
  
  printf("%.15f\n", sum);
}
