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
#include <unordered_map>
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


int calc(int v, ll bits, unordered_map<ll, int> &memo) {
  if (v <= 0) {
    return 0;
  }
  ll hash = bits << 6 ^ v * 6123;
  if (memo.count(hash)) {
    return memo[hash];
  }
  VI pond(v + 1, 0);
  REP(i, 0, v) {
    if (bits & (1LL << i)) {
      continue;
    }
    int res = calc(v - i - 1, bits ^ (1LL << i), memo);
    pond[res] = 1;
  }
  int ret = 0;
  while (ret <= v && pond[ret] >= 1) { ret += 1; }
  assert (ret <= v);
  memo[hash] = ret;
  return ret;
}

const int N = 61;
int dp[N] = {
  0
,1
,1
,2
,2
,2
,3
,3
,3
,3
,4
,4
,4
,4
,4
,5
,5
,5
,5
,5
,5
,6
,6
,6
,6
,6
,6
,6
,7
,7
,7
,7
,7
,7
,7
,7
,8
,8
,8
,8
,8
,8
,8
,8
,8
,9
,9
,9
,9
,9
,9
,9
,9
,9
,9
,10
,10
,10
,10
,10
,10
};

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  int tot = 0;
  if (0) {
    REP(i, 1, N) {
      unordered_map<ll, int> mp;
      int res = calc(i, 0, mp);
      cout << "," << res << endl;
    }
  }
  REP(i, 0, n) {
    tot ^= dp[s[i]];
  }
  cout << (tot == 0 ? "YES" : "NO") << endl;
}
