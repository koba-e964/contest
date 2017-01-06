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

double solve(int n, const VI &cons) {
  if (cons[0] == n) {
    return 0;
  }
  double sum = 1;
  REP(i, 0, n - 1) {
    int p = n - cons[i];
    if (p <= i) {
      return 0;
    }
    sum *= p - i;
    sum /= n - 1 -  i;
  }
  return sum;
}

int main(void){
  int n, s, wid;
  cin >> n >> s >> wid;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  int lim = a[wid] + 100 * s;
  VI cons;
  REP(i, 0, n) {
    if (i == wid) {
      continue;
    }
    int diff = lim - a[i];
    int lo = 0;
    int hi = n; // impossible
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (diff >= 50 * s + 250 * s / (4 + mid)) {
        hi = mid;
      } else {
	lo = mid;
      }
    }
    cons.push_back(hi);
  }
  sort(cons.rbegin(), cons.rend());
  printf("%.15f\n", solve(n, cons));
}
