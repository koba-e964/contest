#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>
 
using namespace std;
 
typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define REP(i,s,n) for(int i=(s); i < int(n); ++i)
 
const int N = 51;
const int W = 1024;
int bb[N];
int n;
 
double dp[N][W];
 
int main(void) {
  int a, b, c;
  cin >> a >> b >> c;
  double aavr = 0;
  REP(i, 0, a) {
    int aa;
    cin >> aa;
    aavr += aa;
  }
  REP(i, 0, b) {
    cin >> bb[i];
  }
  aavr /= a;

  // calculates a table for b
  vector<double> vb(b + 1, 0.0);
  vb[0] = 1;
  REP(i, 0, b) {
    vector<double> nvb(b + 1, 0.0);
    REP(j, 0, b) {
      nvb[j] = vb[j];
    }
    REP(j, 0, b) {
      nvb[j + 1] += vb[j] * bb[i];
    }
    vb = nvb;
  }
  double cur = 1;
  REP(i, 0, b + 1) {
    vb[i] /= cur;
    cur *= b - i;
    cur /= i + 1;
  }

  dp[0][0] = 0;
 
  REP(i, 0, a + 1) {
    REP(j, 0, b + 1) {
      if (i == 0 && j == 0) {
	continue;
      }
      double res = 0;
      if (i >= 1) {
	res += (dp[i - 1][j] + vb[b - j]) * i / (i + j + c);
      }
      if (j >= 1) {
	res += dp[i][j - 1] * j / (i + j + c);
      }
      dp[i][j] = res;
    }
  }
  printf("%.15f\n", dp[a][b] * aavr);
}
