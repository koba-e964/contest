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

int solve(int s, int x1, int x2, int t1, int t2, int p) {
  int onfoot = t2 * abs(x1 - x2);
  int k0 = x1 < x2 ? (x2 - p) * t1 : onfoot;
  int k1 = x1 > x2 ? (2 * s - x2 - p) * t1 : onfoot;
  int k2 = x1 < x2 ? (2 * s + x2 - p) * t1 : onfoot;
  if (x1 < x2) {
    if (x1 < p) {
      k0 = onfoot;
    }
  }
  return min(min(onfoot, k0), min(k1, k2));
}

int main(void){
  int s, x1, x2, t1, t2, p;
  int d;
  cin >> s >> x1 >> x2 >> t1 >> t2 >> p >> d;
  if (t1 >= t2) {
    cout << t2 * abs(x1 - x2) << endl;
    return 0;
  }
  if (d == -1) {
    cout << solve(s, s - x1, s - x2, t1, t2, s - p) << endl;
  } else {
    cout << solve(s, x1, x2, t1, t2, p) << endl;
  }
}
