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
const int DEBUG = 0;

int totient(int m) {
  int s = 0;
  REP(i, 1, m + 1) {
    if (__gcd(i, m) == 1) {
      ++s;
    }
  }
  return s;
}

ll calc(ll a, ll n, int m, bool &small) {
  if (m == 1) {
    return 0;
  }
  if (n == 0) {
    small = true;
    return 1;
  }
  int t = totient(m);
  bool subsmall;
  ll r = calc(a, n - 1, t, subsmall);
  if (subsmall) {
    small = true;
    ll sum = 1;
    REP(i, 0, r) {
      sum *= a;
      if (sum >= m) { small = false; }
      sum %= m;
    }
    return sum;
  }
  small = false;
  ll sum = 1;
  ll cur = a;
  r += t;
  while (r > 0) {
    if (r % 2 != 0) {
      sum *= cur;
      sum %= m;
    }
    cur *= cur;
    cur %= m;
    r /= 2;
  }
  if (DEBUG) {
    cout << "calc(" << a << ", " << n << ", " << m << ") = " << sum << endl;
  }
  return sum;
}

int main(void){
  int a, n, m;
  cin >> a >> n >> m;
  bool t;
  cout << calc(a, n, m, t) << endl;
}
