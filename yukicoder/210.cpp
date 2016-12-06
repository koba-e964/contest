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



int main(void){
  int n;
  cin >> n;
  vector<double> p(n), q(n);
  REP(i, 0, n) {
    cin >> p[i];
  }
  REP(i, 0, n) {
    cin >> q[i];
    q[i] /= 100;
  }
  double p_tot = 1000;
  double tot = 0;
  double overall = 1;
  REP(cnt, 0, 1000000) {
    int maxi = -1;
    double ma = 0;
    REP(i, 0, n) {
      if (ma < p[i] * q[i]) {
	maxi = i;
	ma = p[i] * q[i];
      }
    }
    if (ma < 1e-9) {
      break;
    }
    double sel_p = p[maxi] / p_tot * q[maxi];
    tot += (1 + cnt) * overall * sel_p;
    double tmp = p[maxi];
    p[maxi] *= 1 - q[maxi];
    overall *= 1 - sel_p;
    p_tot -= tmp * q[maxi];
  }
  printf("%.15f\n", tot);
}
