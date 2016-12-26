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

int pd[6] = {2,3,5,7,11,13};
int cd[6] = {4,6,8,9,10,12};

VL calc(int dis[6], int len, int pos, int rem, map<PI, VL> &memo) {
  if (memo.count(PI(pos, rem))) {
    return memo[PI(pos, rem)];
  }
  memo[PI(pos, rem)] = VL();
  VL ret(len, 0);
  if (pos >= 6) {
    ret[0] = rem > 0 ? 0 : 1;
    return memo[PI(pos, rem)] = ret;
  }
  REP(i, 0, rem + 1) {
    VL sub = calc(dis, len, pos + 1, rem - i, memo);
    REP(j, 0, len) {
      if (j + dis[pos] * i < len) {
	ret[j + dis[pos] * i] += sub[j];
	ret[j + dis[pos] * i] %= mod;
      }
    }
  }
  return memo[PI(pos, rem)] = ret;
}

VL get_die(int p, int c) {
  int l = 13 * p + 12 * c + 1;
  VL ret(l, 0);
  map<PI, VL> empty;
  VL sub1 = calc(pd, 13 * p + 1, 0, p, empty);
  empty.clear();
  VL sub2 = calc(cd, 12 * c + 1, 0, c, empty);
  REP(i, 0, 13 * p + 1) {
    REP(j, 0, 12 * c + 1) {
      ret[i + j] += sub1[i] * sub2[j];
      ret[i + j] %= mod;
    }
  }
  return ret;
}

VL poly_mul(const VL &A, const VL &B, const VL &C) {
  int n = A.size();
  int m = B.size();
  assert (C.size() == n);
  VL result(n + m - 1);
  REP(i, 0, n) {
    REP(j, 0, m) {
      result[i + j] += A[i] * B[j];
      result[i + j] %= mod;
    }
  }
  for (int i = n + m - 2; i >= n; --i) {
    REP(j, 0, n) {
      result[i - n + j] += result[i] * C[j];
      result[i - n + j] %= mod;
    }
    result[i] = 0;
  }
  return VL(result.begin(), result.begin() + n);
}

VL poly_pow(const VL &mat, ll e, const VL &mod_poly) {
  int n = mat.size();
  VL sum(n);
  sum[0] = 1;
  VL cur = mat;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = poly_mul(cur, sum, mod_poly);
    }
    cur = poly_mul(cur, cur, mod_poly);
    e /= 2;
  }
  return sum;
}
const int DEBUG = 0;

int main(void){
  ll n;
  int p, c;
  cin >> n >> p >> c;
  VL die = get_die(p, c);
  int len = 13 * p + 12 * c;
  VL poly(len, 0);
  poly[1] = 1;
  VL res(len);
  VL mod_poly(die.rbegin(), die.rend() - 1);
  VL pw = poly_pow(poly, n - 1, mod_poly);
  VL x(2);
  x[1] = 1;
  REP(i, 0, len) {
    res[len - 1 - i] = pw[len - 1];
    pw = poly_mul(pw, x, mod_poly);
  }
  ll tot = 0;
  REP(i, 0, len) {
    REP(j, i + 1, len + 1) {
      tot += res[i] * die[j];
      tot %= mod;
    }
  }
  cout << tot << endl;
}
