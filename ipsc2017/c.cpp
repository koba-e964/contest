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

const int N = 3e5;
double fact[N];

void init(void) {
  fact[0] = 0;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] + log(i);
  }
}

double comb(int x, int y) {
  return fact[x] - (fact[y] + fact[x - y]);
}

int main(void){
  init();
  int t;
  cin >> t;
  while (t--) {
    int n, m; ll a, b;
    cin >> n >> a >> b >> m;
    VI x(n);
    int xtot = 0;
    REP(i, 0, n) {
      cin >> x[i];
      xtot += x[i];
    }
    sort(x.begin(), x.end());
    assert (m > 0); // c1.in
    double num = 0.0;
    double denom = 0.0;
    REP(d, n, m + 1) {
      double p = comb(d, n) - xtot * log(d); // log(p)
      p += fact[xtot];
      REP(i, 0, n) {
	p -= fact[x[i]];
      }
      double ep = exp(p);
      if (a <= d && d <= b) {
	num += ep;
      }
      denom += ep;
    }
    printf("%.15g\n", num / denom);
  }
}
