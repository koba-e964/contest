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
const int DEBUG = 1;

const int N = 51;
string s[N];
int n, m;
pair<int, VL> check(int row) {
  int k = 0;
  VL t;
  int cur = 0;
  int cnt = 0;
  REP(i, 0, m + 1) {
    if (i < m && s[row][i] != '.') {
      if (cnt == 0) {
	cur = i;
      }
      cnt++;
    } else {
      k += cnt / 2;
      if (cnt % 2 != 0) {
	ll acc = 0;
	REP(j, 0, cnt / 2 + 1) {
	  acc |= 1LL << (cur + 2 * j);
	}
	t.push_back(acc);
      }
      cnt = 0;
    }
  }
  return pair<int, VL>(k, t);
}

/**
 * Dinic's algorithm for maximum flow problem.
 * Header requirement: vector, queue
 * Verified by: ABC010-D(http://abc010.contest.atcoder.jp/submissions/602810)
 */
class Dinic {
private:
    struct edge {
        int to, cap, rev; // rev is the position of reverse edge in graph[to]
    };
    std::vector<std::vector<edge> > graph;
    std::vector<int> level;
    std::vector<int> iter;
    /* Perform bfs and calculate distance from s */
    void bfs(int s) {
        level.assign(level.size(), -1);
        std::queue<int> que;
        level[s] = 0;
        que.push(s);
        while (! que.empty()) {
            int v = que.front(); que.pop();
            for (int i = 0; i < graph[v].size(); ++i) {
                edge &e = graph[v][i];
                if (e.cap > 0 && level[e.to] == -1) {
                    level[e.to] = level[v] + 1;
                    que.push(e.to);
                }
            }
        }
    }
    /* search augment path by dfs.
     if f == -1, f is treated as infinity. */
    int dfs(int v, int t, int f) {
        if (v == t) {
            return f;
        }
        for (int &i = iter[v]; i < graph[v].size(); ++i) {
            edge &e = graph[v][i];
            if (e.cap > 0 && level[v] < level[e.to]) {
                int newf = f == -1 ? e.cap : std::min(f, e.cap);
                int d = dfs(e.to, t, newf);
                if (d > 0) {
                    e.cap -= d;
                    graph[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        return 0;
    }
public:
    /* v is the number of vertices (labeled from 0 .. v-1) */
    Dinic(int v) : graph(v), level(v, -1), iter(v, 0) {}
    void add_edge(int from, int to, int cap) {
        graph[from].push_back((edge) {to, cap, graph[to].size()});
        graph[to].push_back((edge) {from, 0, graph[from].size() - 1});
    }
    int max_flow(int s, int t) {
        int flow = 0;
        while (1) {
            bfs(s);
            if (level[t] < 0) {
                return flow;
            }
            iter.assign(iter.size(), 0);
            int f;
            while ((f = dfs(s, t, -1)) > 0) {
                flow += f;
            }
        }
    }
};

int calc(void) {
  Dinic din(n * m + 2);
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == '.') continue;
      int dxy[5] = {1, 0, -1, 0, 1};
      REP(d, 0, 4) {
	int nx = i + dxy[d];
	int ny = j + dxy[d + 1];
	if (nx < 0 || nx >= n || ny < 0 || ny >= m) {
	  continue;
	}
	if (s[nx][ny] != '.') {
	  if ((i + j) % 2) {
	    din.add_edge(i * m + j, nx * m + ny, 1);
	  } else {
	    din.add_edge(nx * m + ny, i * m + j, 1);
	  }
	}
      }
      if ((i + j) % 2) {
	din.add_edge(n * m, i * m + j, 1);
      } else {
	din.add_edge(i * m + j, n * m + 1, 1);
      }
    }
  }
  return din.max_flow(n * m, n * m + 1);
}

int main(void){
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> s[i];
  }
  int w = 0;
  int b = 0;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == 'w') w++;
      if (s[i][j] == 'b') b++;
    }
  }
  int c = calc();
  w -= c;
  b -= c;
  int mi = min(w, b);
  cout << c * 100 + mi * 8 + w + b << endl;
}
