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

const int N = 310;
int dp[N][N][N];

void chmax(int &a, int b) { a = max(a, b); }

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  int k;
  cin >> s >> k;
  int n = s.length();
  REP(i, 0, n) {
    dp[i][i + 1][0] = 1;
  }
  REP(u, 2, n + 1) {
    REP(i, 0, n - u + 1) {
      REP(l, 0, k + 1) {
        int j = i + u;
        int tmp = 0;
        chmax(tmp, dp[i][j - 1][l]);
        chmax(tmp, dp[i + 1][j][l]);
        if (s[i] == s[j - 1]) {
          chmax(tmp, dp[i + 1][j - 1][l] + 2);
        }
        if (l >= 1) {
          chmax(tmp, dp[i + 1][j - 1][l - 1] + 2);
        }
        dp[i][j][l] = tmp;
      }
    }
  }
  int ma = 0;
  REP(i, 0, k + 1) chmax(ma, dp[0][n][i]);
  cout << ma << endl;
}
