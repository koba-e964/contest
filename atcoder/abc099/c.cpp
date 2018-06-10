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
const int N = 100100;
int dp[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI yuki(1, 1);
  int cur = 6;
  while (cur < N) {
    yuki.push_back(cur);
    cur *= 6;
  }
  cur = 9;
  while (cur < N) {
    yuki.push_back(cur);
    cur *= 9;
  }
  REP(i, 1, N) {
    dp[i] = 1e8;
  }
  REP(i, 1, N) {
    for (int v: yuki) {
      if (i < v) continue;
      dp[i] = min(dp[i], dp[i - v] + 1);
    }
  }
  cout << dp[n] << endl;
}
