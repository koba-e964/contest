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


const int N = 0xe869120 / 1e5;
ll dp[N];

int main(void){
  string x, s, t;
  cin >> x >> s >> t;
  dp[0] = 1;
  int n = x.length();
  int sn = s.length();
  int tn = t.length();
  REP(i, 1, n + 1) {
    ll tmp = 0;
    if (i >= sn && x.substr(i - sn, sn) == s) {
      tmp += dp[i - sn];
    }
    if (i >= tn && x.substr(i - tn, tn) == t) {
      tmp += dp[i - tn];
    }
    tmp %= mod;
    dp[i] = tmp;
  }
  cout << dp[n] << endl;
}
