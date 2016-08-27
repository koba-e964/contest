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

const int N = 30;
int n;
int a[N][N];
double comb[N + 1][N + 1];
double prob;
int tot[N];

double dp[N + 1][N + 1];

double memo[N][N];
void vicinity(int tt, int k) {
  double sum = 0;
  REP(j, 0, tt + 1) {
    int l = k - j;
    if (l >= 0 && l <= n - tt - 1) {
      sum += comb[tt][j] * comb[n - tt - 1][l] * pow(prob, j + n - tt - 1 - l) * pow(1 - prob, tt - j + l);
    }
  }
  memo[tt][k] = sum;
}
int main(void){
  int p, q;
  cin >> n >> p;
  cin.ignore();
  cin >> q;
  prob = (double) p / q;
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> a[i][j];
      tot[i] += a[i][j];
    }
  }
  REP(i, 0, N + 1) {
    comb[i][0] = 1;
    REP(j, 1, i + 1) {
      comb[i][j] = comb[i][j - 1] * (i - j + 1) / j;
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      vicinity(i, j);
      //cerr << "memo[" << i << "," << j << "]=" << memo[i][j] << endl;
    }
  }
  vector<PI> t;
  REP(i, 0, n) {
    t.push_back(PI(tot[i], -i));
  }
  sort(t.begin(), t.end());
  dp[n][n] = 1;
  for (int i = n - 1; i >= 0; --i) {
    int v = -t[i].second;
    REP(j, 0, n + 1) {
      REP(k, j, n + 1) {
	if (k == j && i <= n - 2 && t[i].second > t[i + 1].second) {
	  continue;
	}
	double r = memo[tot[v]][j];
	dp[i][j] += dp[i + 1][k] * r;
      }
      //      cerr << "dp[" << i << "," << j << "] = " << dp[i][j] << endl;
    }
  }
  double sum = 0;
  REP(i, 0, n + 1) {
    sum += dp[0][i];
  }
  printf("%.10f\n", sum);
}
