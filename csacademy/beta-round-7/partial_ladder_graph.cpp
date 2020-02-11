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

/*
 * Requirement: VL
 * Header Requirement: vector
 */

vector<VL> mat_mul(const vector<VL> &A, const vector<VL> &B) {
  int n = A.size();
  int m = B.size();
  int l = B[0].size();
  vector<VL> result(n, VL(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	result[i][k] += A[i][j] * B[j][k];
	result[i][k] %= mod;
      }
    }
  }
  return result;
}

vector<VL> mat_pow(const vector<VL> &mat, ll e) {
  int n = mat.size();
  vector<VL> sum(n, VL(n));
  REP(i, 0, n) { sum[i][i] = 1; }
  vector<VL> cur = mat;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = mat_mul(sum, cur);
    }
    cur = mat_mul(cur, cur);
    e /= 2;
  }
  return sum;
}

// C
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  vector<VL> mat(2, VL(2));
  mat[0][0] = 4;
  mat[0][1] = 1;
  mat[1][0] = 2;
  mat[1][1] = 1;
  vector<VL> p = mat_pow(mat, n - 1);
  cout << (p[0][0] + p[0][1]) % mod << endl;
}
