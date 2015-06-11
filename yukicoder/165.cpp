#include <algorithm>
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
typedef pair<int, int> PI;
const double EPS=1e-9;
const int N = 410;
int x[N], y[N], p[N];
int dp[N][N], dp2[N][N];

int main(void){
  int n, b;
  cin >> n >> b;
  REP(i, 0, n) {
    cin >> x[i] >> y[i] >> p[i];
  }
  set<int> ixp;
  REP(i, 0, n) {
    ixp.insert(x[i]);
  }
  set<int> iyp;
  REP(i, 0, n) {
    iyp.insert(y[i]);
  }
  vector<int> ixv(ixp.begin(), ixp.end());
  vector<int> iyv(iyp.begin(), iyp.end());
  REP(i, 0, n) {
    int pp = lower_bound(ixv.begin(), ixv.end(), x[i]) - ixv.begin();
    int qq = lower_bound(iyv.begin(), iyv.end(), y[i]) - iyv.begin();
    dp[pp + 1][qq + 1] += p[i];
    dp2[pp + 1][qq + 1] ++;
  }
  REP(i, 1, N) {
    REP(j, 0, N) {
      dp[i][j] += dp[i - 1][j];
      dp2[i][j] += dp2[i - 1][j];
    }
  }
  int qx = ixv.size() + 1;
  int qy = iyv.size() + 1;
  int ma = 0;
  REP(i, 0, qx) {
    REP(j, i + 1, qx) {
      int mi = 0;
      int tot = 0;
      int tot2 = 0;
      int l = 0;
      REP(k, 0, qy) {
	while (l < qy && tot <= b) {
	  ma = max(ma, tot2);
	  l++;
	  tot += dp[j][l] - dp[i][l];
	  tot2 += dp2[j][l] - dp2[i][l];
	}
	tot -= dp[j][k] - dp[i][k];
	tot2 -= dp2[j][k] - dp2[i][k];
      }
    }
  }
  cout << ma << endl;
}
