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
  string pat[4] = {"dreamer", "dream", "eraser", "erase"};
  string s;
  cin >> s;
  int n = s.length();
  VI dp(n + 1);
  dp[0] = 1;
  REP(i, 0, n + 1) {
    bool found = 0;
    REP(j, 0, 4) {
      int pl = pat[j].length();
      if (i >= pl && s.substr(i - pl, pl) == pat[j]) {
	dp[i] |= dp[i - pl];
      }
    }
  }
  cout << (dp[n] ? "YES" : "NO") << endl;
}
