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

const int M = 1010;
const int N = 1000010;
int s[M], e[M];
ll f[M];
vector<PI> edges[N];

int main(void){
  int n, m, r;
  cin >> n >> m >> r;
  REP(i, 0, m) {
    cin >> s[i] >> e[i] >> f[i];
    e[i] += r;
    if (e[i] >= n) {
      e[i] = n;
    }
    edges[s[i]].push_back(PI(e[i], f[i]));
  }
  VL dp(n + 1);
  
  for (int i = n - 1; i >= 0; --i) {
    REP(j, 0, edges[i].size()) {
      PI v = edges[i][j];
      dp[i] = max(dp[i], dp[v.first] + v.second);
    }
    dp[i] = max(dp[i], dp[i + 1]);
  }
  cout << dp[0] << endl;
}
