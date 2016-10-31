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



int main(void){
  string s;
  cin >> s;
  int n = s.length();
  VI dp(1 << n, -1);
  dp[0] = 0;
  REP(bits, 1, 1 << n) {
    int ma = -1;
    REP(i, 0, n) {
      if ((bits & 1 << i) == 0) { continue; }
      if (s[i] == '0') { continue; }
      REP(j, i + 1, n) {
	if ((bits & 1 << j) == 0) { continue; }
	REP(k, j + 1, n) {
	  if (s[k] != s[j]) { continue; }
	  if (s[i] == s[j]) { continue; }
	  if ((bits & 1 << k) == 0) { continue; }
	  int prev_bit = bits ^ 1 << i ^ 1 << j ^ 1 << k;
	  if (dp[prev_bit] < 0) { continue; }
	  assert (prev_bit < bits);
	  int num = (s[i] - '0') * 100 + 11 * (s[j] - '0');
	  ma = max(ma, dp[prev_bit] + num);
	}
      }
    }
    dp[bits] = ma;
  }
  int ma = 0;
  REP(bits, 0, 1 << n) { ma = max(ma, dp[bits]); }
  cout << ma << endl;
}
