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

const int LF = 4000;
double lfact[LF];

bool check(int d, int x, ll t) {
  if (lfact[x + d - 1] - lfact[x] - lfact[d - 1] >= log(t) + log(1.001)) {
    return 0;
  }
  ll sum = 1;
  REP(i, 0, d - 1) {
    sum *= x + i + 1;
    sum /= i + 1;
  }
  return sum <= t;
}

int main(void){
  double tmp = 0;
  REP(i, 0, LF) {
    lfact[i] = tmp;
    tmp += log(i + 1);
  }
  int q;
  cin >> q;
  REP(i, 0, q) {
    int d,x;
    ll t;
    cin >> d >> x >> t;
    // x ^ (d-1) <= t?
    cout << (check(d, x, t) ? "AC" : "ZETUBOU") << endl;
  }
}
