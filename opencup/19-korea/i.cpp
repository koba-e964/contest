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

const int W = 5010;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  VI dp(W);
  REP(i, 2, W) {
    set<int> seen;
    REP(j, 0, i - 1) {
      int k = i - 2 - j;
      seen.insert(dp[j] ^ dp[k]);
    }
    int mex = 0;
    while (seen.count(mex))mex++;
    dp[i] = mex;
  }
  if (0) {
  REP(i, 0, 20) {
    cerr << " " << dp[i];
  }
  }
  while (t--) {
    int n;
    cin >> n;
    cout << (dp[n] ? "First" : "Second") << endl;
  }
}
