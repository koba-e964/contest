#include <iostream>
#include <queue>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  int w, l;
  cin >> w >> l;
  VL a(w, -1);
  REP(i, 1, w) cin >> a[i];
  ll tot = 0;
  VL dp(w, 0);
  priority_queue<int> rem;
  REP(i, 1, w) {
    if (i <= l) {
      dp[i] = a[i];
      continue;
    }
    ll st = dp[i - l];
    dp[i - l] = 0;
    rem.push(i);
    while (st > 0 && rem.size() > 0) {
      int idx = rem.top(); rem.pop();
      // cerr << "i = " << i << " idx = " << idx << endl;
      if (idx <= i - l) break;
      ll lim = a[idx] - dp[idx];
      lim = min(lim, st);
      dp[idx] += lim;
      st -= lim;
      if (a[idx] > dp[idx]) rem.push(idx);
    }
  }
  REP(i, 0, w) tot += dp[i];
  cout << tot << endl;
}
