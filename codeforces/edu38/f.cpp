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

const int DEBUG = 1;

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 5100;
const int B = 12;
int dp[N][1 << B]; // possibility
int emp[N][1 << B];

const int NOV = 0;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  if (NOV) {
    int nn;
    cin >> nn;
    s = string(nn, 'a');
  } else {
    cin >> s;
  }
  int n = s.length();
  int k = 0;
  while (1 << k <= n) k += 1;
  k -= 1;
  if (DEBUG) {
    DEBUGP(k);
  }
  emp[n][0] = 1;
  for (int i = n - 1; i >= 0; --i) {
    int ch = s[i] - 'a';
    REP(b, 0, k) {
      if (i + (1 << b) <= n) {
	REP(bits, 0, 1 << k) {
	  if ((bits & 1 << b) != 0) {
	    int pbits = bits ^ 1 << b;
	    emp[i][bits] |= emp[i + (1 << b)][pbits];
	    dp[i][bits] |= dp[i + (1 << b)][pbits];
	  }
	}
      }
    }
    REP(bits, 0, 1 << k) {
      if (dp[i + 1][bits] || emp[i + 1][bits])
	dp[i][bits] |= 1 << ch;
    }
    REP(bits, 0, 1 << k)
      dp[i][bits] = dp[i][bits] & -dp[i][bits];
    if (DEBUG) {
      cerr << "dp[" << i << "]:";
      REP(bits, 0, 1 << k) {
	cerr << " " << dp[i][bits];
      }
      cerr << endl;
    }
  }
  string ans;
  int cur = 0;
  int bits = (1 << k) - 1;
  while (cur < n) {
    if (DEBUG) {
      DEBUGP(cur);
      DEBUGP(bits);
      DEBUGP(ans);
    }
    if (emp[cur][bits]) {
      break;
    }
    int idx = -1;
    REP(i, 0, 26) {
      if (dp[cur][bits] & 1 << i) {
	idx = i;
	break;
      }
    }
    assert (idx >= 0);
    char ch = (char)('a' + idx);
    // ??? Greedy always works?
    if (s[cur] == ch && ((dp[cur + 1][bits]) || emp[cur + 1][bits])) {
      cur += 1;
      ans += ch;
      continue;
    }
    bool found = false;
    REP(b, 0, k) {
      if ((bits & 1 << b) == 0) continue;
      if (dp[cur + (1 << b)][bits ^ 1 << b] & 1 << idx) {
	bits ^= 1 << b;
	cur += 1 << b;
	found = true;
	break;
      }
    }
    assert (found);
  }
  cout << ans << "\n";
}
