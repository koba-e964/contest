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

typedef vector<vector<double> > matrix;
matrix mul(const matrix &a, const matrix &b) {
  int n = a.size();
  int m = a[0].size();
  int l = b[0].size();
  assert (m == b.size());
  matrix ret(n);
  REP(i, 0, n) {
    ret[i] = vector<double>(l, 0.0);
    REP(k, 0, l) {
      REP(j, 0, m) {
	ret[i][k] += a[i][j] * b[j][k];
      }
    }
  }
  return ret;
}

int main(void){
  int p0, q;
  cin >> p0 >> q;
  vector<vector<double> > mat(101);
  vector<vector<double> > vec(101);
  // f(p) = p/2 * (1 + f(p - q)) + (1-p)/3 * (1 + f(p + q))
  // build matrix
  double qr = q / 100.0;
  REP(i, 0, 101) {
    double pr = i / 100.0;
    mat[i] = vector<double>(101, 0.0);
    mat[i][max(0, i - q)] += pr / 2;
    mat[i][min(100, i + q)] += (1 - pr) / 3;
    vec[i] = vector<double>(1, pr / 2 + (1 - pr) / 3);
  }
  REP(loop_cnt, 0, 100) {
    matrix sqA = mul(mat, mat);
    matrix Ab = mul(mat, vec);
    mat = sqA;
    REP(i, 0, 101) {
      vec[i][0] += Ab[i][0];
    }
  }
  printf("%.10f\n", (vec[p0][0] + 1) / 3);
}
