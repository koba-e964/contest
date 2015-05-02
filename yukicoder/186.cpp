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

int x[3], y[3];

/* m2 is small (<= 10^6) */
int calc(ll x1, ll m1, ll x2, ll m2, ll *x3, ll *m3) {
  ll g = __gcd(m1, m2);
  ll l = m1 / g * m2;
  for (ll i = x1; i < l; i += m1) {
    if (i % m2 == x2) {
      *x3 = i;
      *m3 = l;
      return 1;
    }
  }
  return 0;
}


int main(void){
  REP(i, 0, 3) {
    cin >> x[i] >> y[i];
  }
  ll rx = 0, rm = 1;
  REP(i, 0, 3) {
    if (! calc(rx, rm, x[i], y[i], &rx, &rm)) {
      cout << -1 << endl;
      return 0;
    }
  }
  cout << (rx == 0 ? rm : rx) << endl;
}
