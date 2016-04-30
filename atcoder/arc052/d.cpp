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

ll dig_tot(ll n) {
  ll sum = 0;
  while (n > 0) {
    sum += n % 10;
    n /= 10;
  }
  return sum;
}


const int S = 100000;
ll tbl[S]; // f(n) in editorial

int main(void){
  ll k, m;
  cin >> k >> m;
  ll cnt = 0;
  map<ll, ll> tbl_map;
  REP(i, 0, S) {
    tbl[i] = (i - dig_tot(i)) % k;
    if (tbl_map.count(tbl[i])) {
      tbl_map[tbl[i]] += 1;
    } else {
      tbl_map[tbl[i]] = 1;
    }
  }
  for(ll i = 0; i <  m / S; ++i) {
    // count {b | tbl_s[i] + tbl[b] == 0 (mod k)}
    ll tbl_s_i = ((i * S) - dig_tot(i * S)) % k;
    cnt += tbl_map[(k - tbl_s_i) % k];
  }
  for (ll i = m / S * S; i <= m; ++i) {
    if (dig_tot(i) % k == i % k) {
      cnt++;
    }
  }
  cout << cnt - 1 << endl;
}
