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
typedef pair<int, int> PI;
const double EPS=1e-9;

vector<PI> vc;

const int N = 5010;
const int W = 6000;

int dp[N][W];

int main(void){
  int n, p;
  cin >> n >> p;
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    vc.push_back(PI(a, b));
  }
  sort(vc.begin(), vc.end());
  reverse(vc.begin(), vc.end());
  REP(i, 0, W) {
    dp[0][i] = 0;
  }
  REP(i, 0, n) {
    REP(j, 0, W) {
      int ma = dp[i][j];
      if (j >= vc[i].first && j - vc[i].first <= p) {
	ma = max(ma, dp[i][j - vc[i].first] + vc[i].second);
      }
      dp[i + 1][j] = ma;
    }
  }
  int ma = 0;
  REP(i, 0, W) {
    ma = max(ma, dp[n][i]);
  }
  cout << ma << endl;
}
