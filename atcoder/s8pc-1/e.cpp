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

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}


int main(void){
  int n, q;
  cin >> n >> q;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VI c(q);
  REP(i, 0, q) {
    cin >> c[i];
    c[i]--;
  }
  c.push_back(0);
  // dist
  VL dist(n - 1);
  REP(i, 0, n - 1) {
    dist[i] = powmod(a[i], a[i + 1]);
  }
  VI imos(n);
  int cur = 0;
  REP(i, 0, q + 1) {
    int x = cur < c[i] ? cur : c[i];
    int y = cur < c[i] ? c[i] : cur;
    // x <= y
    imos[x]++;
    imos[y]--;
    cur = c[i];
  }
  REP(i, 0, n - 1) {
    imos[i + 1] += imos[i];
  }
  ll tot = 0;
  REP(i, 0, n - 1) {
    tot += dist[i] * imos[i] % mod;
    tot %= mod;
  }
  cout << tot << endl;
}
