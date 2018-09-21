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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n, m;
  cin >> n >> m;
  if (n > m) swap(n, m);
  // n <= m
  if (n >= 3) {
    cout << n * m / 2 * 2 << endl;
    return 0;
  }
  if (n == 2) {
    ll ans = m >= 4 ? 2 * m : m == 3 ? 4 : 0;
    if (m == 7) ans = 12;
    cout << ans << endl;
    return 0;
  }
  assert (n == 1);
  int r = m % 6;
  cout << m - min(r, 6 - r) << endl;
}
