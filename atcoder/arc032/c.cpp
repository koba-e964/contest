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


const int N = 1000100;
vector<PI> timely[N];
int dp[N];

int main(void){
  int n;
  cin >> n;
  vector<PI> jobs(n);
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    timely[a].push_back(PI(i, b));
    jobs[i] = PI(a, b);
  }
  dp[N - 1] = 0;
  for (int i = N - 2; i >= 0; --i) {
    int ma = dp[i + 1];
    REP(j, 0, timely[i].size()) {
      PI jb = timely[i][j];
      ma = max(ma, 1 + dp[jb.second]);
    }
    dp[i] = ma;
  }
  cout << dp[0] << endl;
  int cur = 0;
  int cnt = dp[0];
  VI qq;
  vector<VI> poss(cnt + 1);
  REP(i, 0, n) {
    PI jb = jobs[i];
    if (dp[jb.first] - dp[jb.second] == 1) {
      poss[dp[jb.first]].push_back(i);
    }
  }
  while (cnt > 0) {
    bool found = 0;
    REP(j, 0, poss[cnt].size()) {
      int idx = poss[cnt][j];
      if (jobs[idx].first >= cur) {
	qq.push_back(idx);
	cur = jobs[idx].second;
	cnt--;
	found = 1;
	break;
      }
    }
    assert (found);
  }
  assert (cnt == 0);
  assert (qq.size() == dp[0]);
  REP(i, 0, qq.size()) {
    cout << qq[i] + 1 << (i == qq.size() - 1 ? "\n" : " ");
  }
}
