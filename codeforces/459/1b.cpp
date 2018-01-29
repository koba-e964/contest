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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int DEBUG = 0;

const int N = 110;
int g[N][N];
int deg[N][N]; // outgoing

int dp[N][N][26];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, N) {
    REP(j, 0, N) {
      g[i][j] = -1;
    }
  }
  REP(i, 0, m) {
    int v, u;
    char c;
    cin >> v >> u >> c;
    v--, u--;
    g[v][u] = c - 'a';
    REP(j, 0, n) {
      deg[v][j] += 1;
    }
  }
  if (DEBUG) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	cerr << " " << g[i][j];
      }
      cerr<<endl;
    }
  }
  queue<PI> que;
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (deg[i][j] == 0) {
	que.push(PI(i, j));
      }
    }
  }
  while (not que.empty()) {
    PI v = que.front(); que.pop();
    int v1 = v.first;
    int v2 = v.second;
    assert (deg[v1][v2] == 0);
    REP(c, 0, 26) {
      dp[v1][v2][c] = 0;
    }
    REP(w, 0, n) {
      if (g[v1][w] == -1) continue;
      REP(c, 0, g[v1][w] + 1) {
	if (dp[v2][w][g[v1][w]] == 0) {
	  dp[v1][v2][c] = 1;
	}
      }
    }
    // take v out of consideration
    REP(w, 0, n) {
      if (g[w][v2] == -1) continue;
      deg[w][v1] -= 1;
      if (deg[w][v1] == 0) {
	que.push(PI(w, v1));
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      cout << (dp[i][j][0] ? "A" : "B");
    }
    cout << "\n";
  }
}
