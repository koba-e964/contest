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
typedef pair<PI, int> PIPI;

const int N = 100100;

int a[N], b[N];
ll c[N];

ll dp[2 * N + 1];

const ll inf = 1e17;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI coord;
  REP(i, 0, n) {
    cin >> a[i] >> b[i] >> c[i];
    coord.push_back(a[i]);
    coord.push_back(b[i]);
  }
  sort(coord.begin(), coord.end());
  coord.erase(unique(coord.begin(), coord.end()), coord.end());
  REP(i, 0, n) {
    a[i] = lower_bound(coord.begin(), coord.end(), a[i]) - coord.begin();
    b[i] = lower_bound(coord.begin(), coord.end(), b[i]) - coord.begin();
  }
  vector<vector<PI> > buc(2 * N + 1);
  REP(i, 0, n) {
    buc[b[i]].push_back(PI(a[i], i));
  }
  REP(i, 0, 2 * N + 1) {
    dp[i] = -inf;
  }
  dp[0] = 0;
  REP(i, 1, 2 * N + 1) {
    dp[i] = max(dp[i], dp[i - 1]);
    for (auto &e: buc[i]) {
      int from = e.first;
      dp[i] = max(dp[i], dp[from] + c[e.second]);
    }
  }
  ll ma = 0;
  REP(i, 0, 2 * N + 1) {
    ma = max(ma, dp[i]);
  }
  cout << ma << endl;
}
