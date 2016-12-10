#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
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

const int N = 3010;
ll dp[N][N];


const int DEBUG = 0;


int main(void){
  int n, m;
  cin >> n >> m;
  string s;
  cin >> s;
  VI que(n, 0);
  REP(i, 0, n) {
    que[i] = i;
  }
  REP(i, 0, m) {
    int l, r;
    cin >> l >> r;
    l--, r--;
    que[l] = max(que[l], r);
  }
  REP(i, 0, n - 1) {
    que[i + 1] = max(que[i + 1], que[i]);
  }
  VI zs(n + 1), os(n + 1);
  REP(i, 0, n) {
    zs[i + 1] = zs[i] + (s[i] == '0');
    os[i + 1] = os[i] + (s[i] == '1');
  }
  dp[0][0] = 1;
  REP(i, 1, n + 1) {
    int end = que[i - 1];
    int zero = zs[end + 1];
    int one = os[end + 1];
    if (DEBUG) {
      cerr << "[" << i << "]:" << zero << " " << one << endl;
    }
    REP(j, 0, i + 1) {
      ll &ret = dp[i][j];
      // adds zero
      if (j <= zero && j >= 1) {
	ret += dp[i - 1][j - 1];
	ret %= mod;
      }
      // adds one
      if (i - j <= one && j <= i - 1) {
	ret += dp[i - 1][j];
	ret %= mod;
      }
    }
    if (DEBUG) {
      cerr << "dp[" << i << "]:";
      REP(j, 0, i + 1) { cerr << " " << dp[i][j]; }
      cerr << endl;
    }
  }
  ll tot = 0;
  REP(i, 0, n) {
    tot = (tot + dp[n][i]) % mod;
  }
  cout << tot << endl;
}
