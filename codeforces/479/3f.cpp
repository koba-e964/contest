#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VI dp(n, 1), pre(n, -1);
  map<ll, int> seen;
  REP(i, 0, n) {
    if (seen.count(a[i] - 1)) {
      dp[i] = dp[seen[a[i] - 1]] + 1;
      pre[i] = seen[a[i] - 1];
    }
    seen[a[i]] = i;
  }
  VI ans;
  PI ma(-1, -1);
  REP(i, 0, n) ma = max(ma, PI(dp[i], i));
  int rem = ma.first;
  int cur = ma.second;
  while (rem > 0) {
    ans.push_back(cur);
    if (rem == 0) break;
    rem--;
    cur = pre[cur];
  }
  reverse(ans.begin(), ans.end());
  cout << ans.size() << endl;
  REP(i, 0, ans.size()) {
    cout << ans[i] + 1 << (i == (int) ans.size() - 1 ? "\n" : " ");
  }
}
