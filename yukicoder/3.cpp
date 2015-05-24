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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 100100;
const int inf = 123456789;
int n;
int dp[N];

int main(void){
  cin >> n;
  REP(i, 0, n + 1) {
    dp[i] = inf;
  }
  priority_queue<PI, vector<PI>, greater<PI> > que;
  que.push(PI(0, 1));
  dp[1] = 0;
  while (! que.empty()) {
    PI p = que.top(); que.pop();
    int v = p.second;
    if (dp[v] < p.first) {
      continue;
    }
    int dist = __builtin_popcount(v);
    int kk[2] = {v + dist, v - dist};
    REP(i, 0, 2) {
      if (kk[i] < 1 || kk[i] > n) {
	continue;
      }
      if (dp[kk[i]] > p.first + 1) {
	dp[kk[i]] = p.first + 1;
	que.push(PI(p.first + 1, kk[i]));
      }
    }
  }
  cout << (dp[n] == inf ? -1 : dp[n] + 1) << endl;
}
