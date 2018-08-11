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

vector<PI> que[1 << 12];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, q;
  cin >> n >> m >> q;
  VI w(n);
  REP(i, 0, n) cin >> w[i];
  vector<string> s(m);
  REP(i, 0, m) cin >> s[i];
  VI freq(1 << 12);
  REP(i, 0, m) {
    int val = 0;
    REP(j, 0, n) {
      if (s[i][j] == '1') val |= 1 << j;
    }
    freq[val]++;
  }
  REP(i, 0, q) {
    string t;
    int k;
    cin >> t >> k;
    int val = 0;
    REP(j, 0, n) {
      if (t[j] == '1') val |= 1 << j;
    }
    que[val].push_back(PI(k, i));
  }
  VI ans(q, -1);
  REP(bits, 0, 1 << 12) {
    VI dp(101);
    REP(targ, 0, 1 << 12) {
      int sum = 0;
      int pat = bits ^ targ;
      REP(i, 0, n)
	if ((pat & 1 << i) == 0) sum += w[i];
      if (sum <= 100) dp[sum] += freq[targ];
    }
    REP(i, 0, 100) dp[i + 1] += dp[i];
    REP(j, 0, que[bits].size()) {
      int k = que[bits][j].first;
      int qidx = que[bits][j].second;
      ans[qidx] = dp[k];
    }
  }
  REP(i, 0, q) cout << ans[i] << "\n";
}
