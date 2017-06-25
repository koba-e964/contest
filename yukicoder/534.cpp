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

vector<VL> mul(const vector<VL> &a, const vector<VL> &b, ll mod) {
  int n = a.size();
  int m = b.size();
  int l = b[0].size();
  vector<VL> ret(n, VL(l, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	ret[i][k] = (ret[i][k] + a[i][j] * b[j][k]) % mod;
      }
    }
  }
  return ret;
}

vector<VL> pow(const vector<VL> &a, ll e, ll mod) {
  int n = a.size();
  vector<VL> sum(n, VL(n, 0));
  REP(i, 0, n) {
    sum[i][i] = 1;
  }
  vector<VL> cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = mul(sum, cur, mod);
    }
    cur = mul(cur, cur, mod);
    e /= 2;
  }
  return sum;
}

ll fib(ll n, ll mod) {
  vector<VL> a(2, VL(2, 1));
  a[0][0] = 0;
  vector<VL> ea = pow(a, n, mod);
  return ea[0][1] % mod;
}

int main(void){
  ll n;
  cin >> n;
  if (0) {
    REP(i, 0, 60) {
      cout << i << "~" << fib(i, 7) << endl;
    }
  }
  ll p = fib(n, 2 * (mod + 1));
  cout << fib(p, mod) << endl;
}
