#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
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
  ll n, m;
  cin >> n >> m;
  if (n == 1) {
    cout << (m + 1) % mod << "\n";
    return 0;
  }
  ll tot = (m + 1) % mod;
  ll lim = (m + 1) / (n - 1);
  tot *= (2 * lim + 1) % mod;
  tot %= mod;
  ll tmp = (n - 1) % mod;
  tmp *= lim % mod;
  tmp %= mod;
  tmp *= (lim + 1) % mod;
  tmp %= mod;
  tot = (tot + mod - tmp) % mod;
  cout << tot << "\n";
}
