#include <algorithm>
#include <bitset>
#include <cassert>
#include <cstdio>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


int gauss_elim(vector<vector<double> > &A,
			  vector<double> &b) {
  int n = A.size();
  int m = A[0].size();
  assert (b.size() == n);
  int c = 0;
  // TODO no pivoting
  REP(i, 0, n) {
    while (c < m && A[i][c] == 0) {
      c++;
    }
    if (c >= m) {
      return i;
    }
    double aic = A[i][c];
    A[i][c] = 1;
    REP(j, c + 1, m) {
      A[i][j] /= aic;
    }
    b[i] /= aic;
    REP(j, 0, n) {
      if (i == j) { continue; }
      double ajc = A[j][c];
      A[j][c] = 0;
      REP(k, c + 1, m) {
	A[j][k] -= ajc * A[i][k];
      }
      b[j] -= ajc * b[i];
    }
  }
  return n;
}


int main(void){
  int n, m;
  cin >> n >> m;
  vector<double> c(n, 0);
  REP(i, 1, n - 1) {
    cin >> c[n - 1 - i];
  }
  vector<vector<double> > dp(2, vector<double>(n, 1.0 / 0));
  REP(i, 0, m + 1) {
    dp[0][i] = 0.0;
  }
  // Calculates dp[1][i], by using Gaussian elimination
  vector<vector<double> > A(m, vector<double>(m, 0));
  REP(i, 0, m) {
    A[i][i] = m;
  }
  vector<double> b(m);
  REP(i, 1, m) {
    REP(j, 1, m + 1) {
      A[i][abs(i - j)] -= 1.0;
      b[i] += c[abs(i - j)];
    }
  }
  int result = gauss_elim(A, b);
  assert (result == m);
  REP(i, 0, m) {
    dp[1][i] = b[i];
  }
  REP(i, m, n) {
    double tot = 0;
    REP(j, 1, m + 1) {
      tot += dp[1][i - j] + c[i - j];
    }
    dp[1][i] = tot / m;
  }
  REP(i, m, n) {
    double tot = 0;
    REP(j, 1, m + 1) {
      tot += dp[0][i - j] + c[i - j];
    }
    double mi = tot / m;
    REP(j, 1, m + 1) {
      mi = min(mi, dp[1][i - j] + c[i - j]);
    }
    dp[0][i] = mi;
  }
  if (0) {
    REP(i, 0, 2) {
      REP(j, 0, n) {
	cerr << "dp[" << i << "," << j << "]=" << dp[i][j] << endl;
      }
    }
  }
  printf("%.15f\n", dp[0][n - 1]);
}
