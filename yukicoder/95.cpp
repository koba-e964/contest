#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

const int inf = 1e7;
const int K = 16;
int dp[1 << K][K];

// Checks if there exists a Hamiltonian path with length <= k.
bool check(const vector<VI> &dist, const VI &route, int k) {
  assert (route[0] == 0);
  int m = route.size();
  assert (m <= k + 1);
  REP(i, 0, 1 << m) {
    REP(j, 0, m) {
      dp[i][j] = inf;
    }
  }
  dp[1][0] = 0;
  for (int bits = 3; bits < 1 << m; bits += 2) {
    REP(i, 0, m) {
      if ((bits & 1 << i) == 0) { continue; }
      REP(j, 0, m) {
	if (i == j || (bits & 1 << j) == 0) { continue; }
	dp[bits][i] = min(dp[bits][i], dp[bits ^ 1 << i][j]
			  + dist[route[i]][route[j]]);
      }
    }
  }
  int mi = inf;
  REP(i, 0, m) {
    mi = min(mi, dp[(1 << m) - 1][i]);
  }
  return mi <= k;
}


// I solved this problem after reading the editorial
int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  vector<VI> dist(n, VI(n, inf));
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    dist[x][y] = 1;
    dist[y][x] = 1;
  }
  REP(i, 0, n) {
    dist[i][i] = 0;
  }
  REP(l, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	dist[i][j] = min(dist[i][j], dist[i][l] + dist[l][j]);
      }
    }
  }
  VI route(1, 0);
  ll tot = 0;
  for (int i = n - 1; i >= 1; --i) {
    if (route.size() >= k + 1) {
      break;
    }
    route.push_back(i);
    if (check(dist, route, k)) {
      tot += (1LL << i) - 1;
    } else {
      route.pop_back();
    }
  }
  assert (route.size() <= k + 1);
  cout << tot << endl;
}
