#include <algorithm>
#include <bitset>
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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 2000;
bitset<N> a[N];
bitset<N> dp[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 1, n) {
    string s;
    cin >> s;
    REP(j, 0, i) {
      a[i][j] = s[j] == '1';
      a[j][i] = s[j] == '0';
    }
  }
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "a[" << i << "]";
      REP(j, 0, n) {
        cerr << a[i][j];
      }
      cerr << endl;
    }
  }
  REP(i, 0, n) {
    dp[i][i] = true;
  }
  REP(s, 1, n) {
    REP(i, 0, n - s) {
      int j = i + s;
      bool res = ((dp[i] & dp[j - 1]) & a[j]).any();
      dp[i][j] =res;
      res = ((dp[i + 1] & dp[j]) & a[i]).any();
      dp[j][i] = res;
    }
  }
  int ans = 0;
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "dp[" << i << "]";
      REP(j, 0, n) {
        cerr << dp[i][j];
      }
      cerr << endl;
    }
  }
  REP(i, 0, n) {
    ans += dp[0][i] && dp[n - 1][i] ? 1 : 0;
  }
  cout << ans << endl;
}
