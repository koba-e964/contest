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

const int N = 210;
const int M = 1010;

int n, m;
int u[N];
vector<PI> edges[N];
ll dp[N][M];
ll tmp[M];

void rec(int v, int par) {
  fill_n(dp[v], M, 0);
  
  REP(i, 0, edges[v].size()) {
    int next = edges[v][i].first;
    int time = edges[v][i].second;
    if (par == next) {
      continue;
    }
    rec(next, v);
    copy_n(dp[v], M, tmp);
    REP(j, 0, M - time) {
      REP(k, 0, M - time - j) {
	int next_time = j + k + time;
	ll next_tax = dp[v][j] + dp[next][k];
	tmp[next_time] = max(tmp[next_time], next_tax);
      }
    }
    copy_n(tmp, M, dp[v]);
  }
  REP(i, 0, M) {
    dp[v][i] += u[v];
  }
}

int main(void){
  cin >> n >> m;
  m /= 2;
  REP(i, 0, n) {
    cin >> u[i];
  }
  REP(i, 0, n - 1) {
    int a, b, c;
    cin >> a >> b >> c;
    edges[a].push_back(PI(b, c));
    edges[b].push_back(PI(a, c));
  }
  rec(0, -1);
  ll ma = 0;
  REP(i, 0, m + 1) {
    ma = max(dp[0][i], ma);
  }
  cout << ma << endl;
}
