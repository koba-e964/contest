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

const int N = 301;

int dist[N][2 * N];
const int inf = 1e9;

const int T = 10500;
VI edges[T];

int main(void){
  int m, n;
  cin >> m >> n;
  REP(i, 0, N) {
    REP(j, 0, 2 * N) {
      dist[i][j] = inf;
    }
  }
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    REP(i, a, N) {
      if (i + b - a < 2 * N) {
	dist[i][i + b - a] = 1;
      }
    }
  }
  REP(i, 0, N) {
    REP(j, 0, 2 * N) {
      if (dist[i][j] != inf) {
	edges[i].push_back(j);
      }
    }
  }
  REP(i, N, T) {
    REP(j, 0, edges[N - 1].size()) {
      int w = edges[N - 1][j];
      if (i + w - N + 1 < T) {
	edges[i].push_back(i + w - N + 1);
      }
    }
  }
  queue<PI> que;
  VI tbl(T, inf);
  que.push(PI(1, 0));
  while (not que.empty()) {
    PI p = que.front(); que.pop();
    int v = p.first;
    if (tbl[v] != inf) {
      continue;
    }
    tbl[v] = p.second;
    REP(j, 0, edges[v].size()) {
      int w = edges[v][j];
      if (tbl[w] == inf) {
	que.push(PI(w, p.second + 1));
      }
    }
  }
  cout << (tbl[n] == inf ? -1 : tbl[n] + 1) << endl;
}
