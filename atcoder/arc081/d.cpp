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



int main(void) {
  int n;
  string s1, s2;
  cin >> n >> s1 >> s2;
  VL dp(n + 1);
  int pos = 0;
  while (pos < n) {
    if (s1[pos] == s2[pos]) {
      ll fac = pos == 0 ? 0 :
	s1[pos - 1] == s2[pos - 1] ? 2 : 1;
      dp[pos + 1] = pos == 0 ? 3 : dp[pos] * fac % mod;
      pos += 1;
    } else {
      ll fac = pos == 0 ? 0 :
	s1[pos - 1] == s2[pos - 1] ? 2 : 3;
      dp[pos + 2] = pos == 0 ? 6 : dp[pos] * fac % mod;
      pos += 2;
    }
  }
  cout << dp[n] << endl;
}
