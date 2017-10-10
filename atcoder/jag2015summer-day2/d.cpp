#include <iostream>
#include <map>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;
typedef pair<int, int> PI;


int main(void) {
  int n, m, k;
  cin >> n >> m >> k;
  VI d(m);
  map<int, int> d_inv;
  REP(i, 0, m) {
    cin >> d[i];
    d[i]--;
    d_inv[d[i]] = i;
  }
  vector<VI> edges(n);
  REP(i, 0, n) {
    REP(j, 0, k) {
      int v;
      cin >> v;
      v--;
      edges[i].push_back(v);
    }
  }
  // Powerset construction - 2^m vertices
  vector<VI> ps(1 << m, VI(k, 0));
  REP(i, 0, k) {
    REP(j, 0, m) {
      int w = edges[d[j]][i];
      if (d_inv.count(w)) {
	int tmp = 1 << d_inv[w];
	REP(bits, 0, 1 << m) {
	  if ((bits & 1 << j) == 0) { continue; }
	  ps[bits][i] |= tmp;
	}
      }
    }
  }
  queue<PI> que;
  vector<int> dist(1 << m, 1e8);
  que.push(PI(0, (1 << m) - 1));
  while (not que.empty()) {
    PI t = que.front(); que.pop();
    int d = t.first;
    int v = t.second;
    if (dist[v] <= d) { continue; }
    dist[v] = d;
    REP(i, 0, k) {
      que.push(PI(d + 1, ps[v][i]));
    }
  }
  cout << dist[0] << endl;
}
