#include <algorithm>
#include <cassert>
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

const int minf = -1e9;
const int N = 5010, M = 10;
int dp[2][M + 1][1 << M];
char c[M];
int rev_c[26];
int p[M];
string s;
int main(void){
  int n, m, y, z;
  cin >> n >> m >> y >> z;
  REP(i, 0, m) {
    cin >> c[i] >> p[i];
    rev_c[c[i] - 'A'] = i;
  }
  cin >> s;
  REP(i, 0, 2) {
    REP(j, 0, M + 1) {
      REP(k, 0, 1 << M) {
	dp[i][j][k] = minf;
      }
    }
  }
  dp[0][m][0] = 0;
  REP(i, 1, n + 1) {
    int t = i % 2;
    REP(j, 0, M + 1) {
      REP(k, 0, 1 << M) {
	dp[t][j][k] = minf;
      }
    }
    int cur = rev_c[s[i - 1] - 'A'];
    assert (0 <= cur && cur < m);
    REP(j, 0, m + 1) {
      REP(bits, 0, 1 << m) {
	int ma = dp[1 - t][j][bits];
	if (j == cur && (bits & (1 << cur)) != 0) {
	  REP(k, 0, m + 1) {
	    ma = max(ma, dp[1-t][k][bits ^ 1<<cur] + p[cur] + (k == j ? y : 0));
	    ma = max(ma, dp[1-t][k][bits] + p[cur] + (k == j ? y : 0));
	  }
	}
	dp[t][j][bits] = ma;
      }
    }
  }
  int ma = minf;
  REP(j, 0, m + 1) {
    REP(bits, 0, 1 << m) {
      ma = max(ma, dp[n%2][j][bits] + (bits == ((1<<m) - 1) ? z : 0));
    }
  }
  cout << ma << endl;
}
