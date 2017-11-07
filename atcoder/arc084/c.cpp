#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n), c(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  REP(i, 0, n) {
    cin >> c[i];
  }
  sort(a.begin(), a.end());
  sort(b.begin(), b.end());
  sort(c.begin(), c.end());
  VL bc(n);
  REP(i, 0, n) {
    bc[i] = c.end() - upper_bound(c.begin(), c.end(), b[i]);
  }
  for (int i = n - 2; i >= 0; --i) {
    bc[i] += bc[i + 1];
  }
  VL ac(n);
  ll tot = 0;
  REP(i, 0, n) {
    int tmp = upper_bound(b.begin(), b.end(), a[i]) - b.begin();
    if (tmp < n) {
      tot += bc[tmp];
    }
  }
  cout << tot << "\n";
}
