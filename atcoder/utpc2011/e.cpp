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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int W = 1234567;
int dp[W];

// dp[i + 1][j] = max(dp[i][j], dp[i][j - a[i]] + 1 if j <= b[i])

int main(void) {
  int n;
  cin >> n;
  vector<PI> pt;
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    pt.push_back(PI(b, a)); // b first!! This vector should be sorted by b!!
  }
  sort(pt.begin(), pt.end());
  REP(i, 0, n) {
    int b = pt[i].first;
    int a = pt[i].second;
    if (a > b) {
      continue;
    }
    // ONEGAI O(nW) solution
    for (int j = b; j >= a; --j) {
      dp[j] = max(dp[j], dp[j - a] + 1);
    }
  }
  int ma = 0;
  REP(i, 0, W) {
    ma = max(ma, dp[i]);
  }
  cout << ma << endl;
}
