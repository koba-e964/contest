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

vector<VL> mul(const vector<VL> &A, const vector<VL> &B) {
  int n = A.size();
  int m = B.size();
  int l = B[0].size();
  vector<VL> res(n, VL(l));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	res[i][k] += A[i][j] * B[j][k];
	res[i][k] %= mod;
      }
    }
  }
  return res;
}

ll powmod(ll x, ll e) {
  ll sum = 1;
  ll cur = x % mod;
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
  int n;
  ll c;
  cin >> n >> c;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  vector<VL > A(n, VL(n));
  REP(i, 0, n) {
    REP(j, i, n) {
      A[i][j] = a[j];
    }
  }
  vector<VL> sum(n, VL(n)), cur;
  cur = A;
  REP(i, 0, n) {
    sum[i][i] = 1;
  }
  ll oldc = c;
  while (c > 0) {
    if (c % 2) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    c /= 2;
  }
  ll res = 0;
  REP(i, 0, n) {
    res = (res + sum[0][i]) % mod;
    res = (res - powmod(a[i], oldc) + mod) % mod;
  }
  cout << res << endl;
}
