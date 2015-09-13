#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

vector<PI> rows, cols;
const int inf = 1e6;

const int T = 100100;

int solve(int s, const vector<PI> &conf) {
  VI dp(s);
  VI events[T];
  REP(i, 0, s) {
    dp[i] = 0;
  }
  REP(i, 0, conf.size()) {
    PI t = conf[i];
    events[t.first].push_back(t.second);
  }
  REP(i, 0, T) {
    sort(events[i].begin(), events[i].end());
    // from left to right 
    REP(j, 0, events[i].size()) {
      int x = events[i][j];
      dp[x] = x == 0 ? inf : dp[x - 1] + 1;
    }
    for (int j = events[i].size() - 1; j >= 0; --j) {
      int x = events[i][j];
      dp[x] = min(dp[x], x == s - 1 ? inf : dp[x + 1] + 1);
    }
  }
  int mi = inf;
  REP(x, 0, s) {
    mi = min(dp[x], mi);
  }
  return mi;
}


int main(void){
  int w, h, q;
  cin >> w >> h >> q;
  REP(i, 0, q) {
    int t, d, x;
    cin >> t >> d >> x;
    t--;
    x--;
    if (d == 0) {
      cols.push_back(PI(t, x));
    } else {
      rows.push_back(PI(t, x));
    }
  }
  int res = solve(h, rows) + solve(w, cols);
  cout << (res >= inf ? -1 : res) << endl; 
}
