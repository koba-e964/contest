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

/**
 * Dijkstra's algorithm.
 * First, call add_edge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Header requirement: algorithm, queue, vector
 * Verified by AtCoder ARC026-C (http://arc026.contest.atcoder.jp/submissions/604231)
 */
 template<class Len = int>
class Dijkstra {
private:
  int n;
  std::vector<std::vector<std::pair<int, Len> > > edges;
public:
  /**
   * n: the number of vertices
   */
  Dijkstra(int n) : n(n), edges(n) {}
  /*
   * from: the source of edge to add
   * to: the target of edge to add
   * cost: the cost of edge to add
   */
  void add_edge(int from, int to, Len cost) {
    edges[from].push_back(std::pair<int, Len>(to, cost));
  }
  /*
   * This function returns an array consisting of the distances from vertex source.
   */
  std::vector<Len> solve(int source) {
    const Len inf = 1e8;
    typedef std::pair<Len, int> pi;
    std::vector<Len> d(n, inf);
    std::priority_queue<pi, std::vector<pi>, std::greater<pi> > que;
    que.push(pi(0, source));
    while (!que.empty()) {
      pi p = que.top(); que.pop();
      int idx = p.second;
      if (d[idx] <= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < (int) edges[idx].size(); ++j) {
	que.push(pi(p.first + edges[idx][j].second, edges[idx][j].first));
      }
    }
    return d;
  }
};


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, PI> PIPI;

const int H = 50;
const int K = 2500;

VI solve(vector<string> s, vector<PIPI> food, int sx, int sy) {
  // Random walk.
  int time = 0;
  vector<VI> mp(H, VI(H, -2));
  vector<VI> vis(H, VI(H, 0));
  REP(i, 0, H) {
    REP(j, 0, H) {
      // mp: # => -2, . => -1, feed => id
      mp[i][j] = s[i][j] == '#' ? -2 : -1;
    }
  }
  int n = food.size();
  REP(i, 0, n) {
    int x = food[i].first / H;
    int y = food[i].first % H;
    assert (mp[x][y] == -1);
    mp[x][y] = i;
  }
  Dijkstra<int> dijk(K);
  vector<VI> adjacent(K);
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  REP(i, 0, H) {
    REP(j, 0, H) {
      if (mp[i][j] == -2) { continue; }
      REP(d, 0, 4) {
	int nx = i + dx[d];
	int ny = j + dy[d];
	if (nx < 0 || nx >= H || ny < 0 || ny >= H) { continue; }
	if (mp[nx][ny] == -2) { continue; }
	dijk.add_edge(i * H + j, nx * H + ny, 1);
	adjacent[i * H + j].push_back(nx * H + ny);
      }
    }
  }
  VI res;
  while (time < K) {
    // Pick the food
    assert (mp[sx][sy] != -2);
    mp[sx][sy] = -1;
    vis[sx][sy] += 1;
    vector<int> dist = dijk.solve(sx * H + sy);
    const ll minf = -1e15;
    ll ma = minf;
    int maxi = -1;
    int mdist = -1;
    REP(i, 0, K) {
      if (dist[i] <= 2 * K && mp[i / H][i % H] >= 0) { // there's some food
	int idx = mp[i / H][i % H];
	int eta = time + dist[i];
	PI f = food[idx].second;
	int pt = f.first - eta * f.second;
	ll qol = pt >  -50000 ? pt - 100000LL * dist[i] : minf;
	if (ma < qol) {
	  ma = qol;
	  maxi = i;
	  mdist = dist[i];
	}
      }
    }
    if (maxi == -1) {
      break;
    }
    int cur = maxi;
    VI path;
    REP(i, 0, mdist) {
      int cx = cur / H;
      int cy = cur % H;
      assert (mp[cx][cy] != -2);
      mp[cx][cy] = -1;
      vector<PI> adj;
      int eta = time + mdist - i - 1;
      REP(d, 0, 4) {
	int ax = cx - dx[d];
	int ay = cy - dy[d];
	if (ax < 0 || ax >= H || ay < 0 || ay >= H) { continue; }
	if (mp[ax][ay] == -2) { continue; }
	if (dist[ax * H + ay] == dist[cur] - 1) {
	  int idx = mp[ax][ay];
	  int sc = 0;
	  if (idx >= 0) {
	    PI f = food[idx].second;
	    sc = f.first - eta * f.second;
	  }
	  adj.push_back(PI(sc, d));
	}
      }
      sort(adj.rbegin(), adj.rend());
      assert (adj.size() >= 1);
      int d = adj[0].second;
      path.push_back(d);
      cur = (cx - dx[d]) * H + (cy - dy[d]);
    }
    assert ((int) path.size() == mdist);
    assert (cur == sx * H + sy);
    //cerr << "path, ma = " << ma << ", dist = " << mdist << endl;
    reverse(path.begin(), path.end());
    REP(i, 0, mdist) {
      res.push_back(path[i] + 1);
    }
    time += mdist;
    assert ((int) res.size() == time);
    sx = maxi / H;
    sy = maxi % H;
  }
  while (res.size() < K) { res.push_back(0); }
  return res;
}

vector<int> decorate(vector<string> s, vector<PIPI> food, int sx, int sy,
		     VI result) {
  int ma = 0;
  int mtime = -1;
  vector<VI> mp(H, VI(H, -2));
  REP(i, 0, H) {
    REP(j, 0, H) {
      // mp: # => -2, . => -1, feed => id
      mp[i][j] = s[i][j] == '#' ? -2 : -1;
    }
  }
  int n = food.size();
  REP(i, 0, n) {
    int x = food[i].first / H;
    int y = food[i].first % H;
    assert (mp[x][y] == -1);
    mp[x][y] = i;
  }
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  int tot = 0;
  REP(time, 0, K) {
    if (result[time] == 0) { continue; }
    int d = result[time] - 1;
    assert (d < 4);
    int nx = sx + dx[d];
    int ny = sy + dy[d];
    assert (nx >= 0 && nx < H && ny >= 0 && ny < H);
    assert (mp[nx][ny] != -2);
    if (mp[nx][ny] >= 0) {
      int idx = mp[nx][ny];
      PI f = food[idx].second;
      tot += f.first - f.second * (time + 1);
    }
    sx = nx;
    sy = ny;
    if (ma < tot) {
      ma = tot;
      mtime = time;
    }
  }
  REP(i, mtime + 1, K) {
    result[i] = 0;
  }
  return result;
}

int main(void){
  int h, w, k, sr, sc;
  cin >> h >> w >> k >> sr >> sc;
  sr--, sc--;
  vector<string> s(H);
  REP(i, 0, H) {
    cin >> s[i];
  }
  int n;
  cin >> n;
  vector<PIPI> food;
  REP(i, 0, n) {
    int fr, fc, f, d;
    cin >> fr >> fc >> f >> d;
    fr--, fc--;
    food.push_back(PIPI(fr * H + fc, PI(f, d)));
  }
  VI result = solve(s, food, sr, sc);
  VI result2 = decorate(s, food, sr, sc, result);
  REP(i, 0, K) {
    cout << "-RULD"[result2[i]];
  }
  cout << endl;
}
