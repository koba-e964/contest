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

const int N = 800;
const int BIAS = 400;
int dp[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  vector<PI> s(4), t(4);
  REP(i, 0, 4) {
    int x, y;
    cin >> x >> y;
    s[i] = PI(x, y);
  }
  REP(i, 0, 4) {
    int x, y;
    cin >> x >> y;
    t[i] = PI(x, y);
  }
  sort(s.begin(), s.end());
  sort(t.begin(), t.end());
  bool ok = false;
  int x = s[1].second - s[0].second;
  int y = -(t[1].second - t[0].second);
  REP(i, 0, x + 1) {
    REP(j, 0, x + 1) {
      dp[s[0].first + i + BIAS][s[0].second + j + BIAS]++;
    }
  }
  REP(i, -y, y + 1) {
    REP(j, -y, y + 1) {
      if (abs(i) + abs(j) <= y) {
        dp[t[1].first + i + BIAS][t[0].second + j + BIAS]++;
      }
    }
  }
  REP(i, 0, N) REP(j, 0, N) {
    if (dp[i][j] >= 2) ok = true;
  }
  cout << (ok ? "Yes" : "No") << "\n";
}
