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
const int DEBUG = 0;


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


int solve(int d, const vector<double> &v) {
  REP(i, 0, d + 2) {
    vector<vector<double> > A(d + 1, vector<double>(d + 1, 0));
    vector<double> b(d + 1);
    int pos = 0;
    REP(j, 0, d + 2) {
      if (i == j) { continue; }
      // A[pos]
      double pw = 1;
      REP(k, 0, d + 1) {
	A[pos][k] = pw;
	pw *= j;
      }
      b[pos] = v[j];
      pos++;
    }
    int retval = gauss_elim(A, b);
    assert (retval == d + 1);
    double ex_val = 0;
    double pw = 1;
    REP(j, 0, d + 1) {
      ex_val += b[j] * pw;
      pw *= d + 2;
    }
    if (DEBUG) {
      cerr << "ex_val: " << ex_val <<
	"v[" << d + 2 << "]: " << v[d + 2] << endl;
    }
    if (abs(ex_val - v[d + 2]) <= 0.001) {
      return i;
    }
  }
  return d + 2;
}

int main(void){
  int d;
  while ((cin >> d) && d) {
    vector<double> v(d + 3);
    REP(i, 0, d + 3) {
      cin >> v[i];
    }
    cout << solve(d, v) << endl;
  }
}
