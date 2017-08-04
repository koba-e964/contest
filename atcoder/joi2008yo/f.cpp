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

const ll inf = 1e16;
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  vector<VL> dist(n, VL(n, inf));

  REP(loop_count, 0, k) {
    int ty;
    cin >> ty;
    if (ty == 0) {
      int a, b;
      cin >> a >> b;
      a--, b--;
      cout << (dist[a][b] == inf ? -1 : dist[a][b]) << "\n";
      continue;
    }
    int c, d;
    ll e;
    cin >> c >> d >> e;
    c--, d--;
    dist[c][d] = min(dist[c][d], e);
    dist[d][c] = min(dist[d][c], e);
    REP(i, 0, n) {
      REP(j, 0, n) {
	dist[i][j] = min(dist[i][j], dist[i][c] + dist[c][j]);
      }
    }
    REP(i, 0, n) {
      REP(j, 0, n) {
	dist[i][j] = min(dist[i][j], dist[i][d] + dist[d][j]);
      }
    }
  }
}
