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

VL mul(const VL &a, const VL &b) {
  int n = a.size();
  VL ret(2 * n - 1);
  REP(i, 0, n) {
    REP(j, 0, n) {
      ret[i + j] += a[i] * b[j];
      ret[i + j] %= mod;
    }
  }
  for (int i = n - 2; i >= 0; --i) {
    ret[i] += ret[i + n];
    ret[i] %= mod;
  }
  return VL(ret.begin(), ret.begin() + n);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  VL a(n, 0);
  VL sum(n, 0);
  a[0] = 1;
  sum[0] = 1;
  REP(et, 0, m) {
    int p;
    cin >> p;
    a[p % n] += 1;
  }
  VL cur(a);
  while (k > 0) {
    if (k % 2) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    k /= 2;
  }
  REP(i, 0, n) {
    cout << sum[i] << "\n";
  }
}
