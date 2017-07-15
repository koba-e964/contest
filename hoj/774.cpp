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

const int N = 510;
int cost[N][N];
int exha[N][N];
const int inf = 1e8;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    REP(j, 0, n) {
      cost[i][j] = exha[i][j] = inf;
    }
  }
  REP(i, 0, m) {
    int a, b, c, t;
    cin >> a >> b >> c >> t;
    a--, b--;
    cost[a][b] = min(cost[a][b], c);
    cost[b][a] = min(cost[b][a], c);
    exha[a][b] = min(exha[a][b], t);
    exha[b][a] = min(exha[b][a], t);
  }
  REP(k, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	cost[i][j] = min(cost[i][j], cost[i][k] + cost[k][j]);
	exha[i][j] = min(exha[i][j], exha[i][k] + exha[k][j]);
      }
    }
  }
  int d;
  cin >> d;
  REP(i, 0, d) {
    int x, y, e, p;
    cin >> x >> y >> e >> p;
    x--, y--;
    cout << min(cost[x][y] * e, exha[x][y] * p) << "\n";
  }
}
