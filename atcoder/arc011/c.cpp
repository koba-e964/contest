#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;
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
    const Len inf = 1e16;
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
      for(int j = 0; j < edges[idx].size(); ++j) {
	que.push(pi(p.first + edges[idx][j].second, edges[idx][j].first));
      }
    }
    return d;
  }
};

const int N = 1002;
int n, m;
map<string, int> rev;
string s[N];
int dist[N][N];

int reg(const string &str) {
  if (rev.count(str)) {
    return rev[str];
  }
  int idx = rev.size();
  s[idx] = str;
  rev[str] = idx;
  return idx;
}

int main(void){
  string fst, lst;
  cin >> fst >> lst >> n;
  reg(fst);
  reg(lst);
  REP(i, 0, n) {
    string tmp;
    cin >> tmp;
    reg(tmp);
  }
  n = rev.size();
  m = s[0].length();
  Dijkstra<ll> dijk(n);
  REP(i, 0, n) {
    REP(j, 0, i) {
      int diff = 0;
      REP(k, 0, m) {
	if (s[i][k] != s[j][k]) diff++;
      }
      dist[i][j] = dist[j][i] = diff == 1 ? 1 : 10000;
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (dist[i][j] == 1) {
	dijk.add_edge(i, j, 1);
      }
    }
  }
  int fst_idx = reg(fst);
  int lst_idx = reg(lst);
  VL sol = dijk.solve(lst_idx);
  if (sol[fst_idx] == 0) {
    assert (fst == lst);
    cout << 0 << endl;
    cout << fst << endl << lst << endl;
    return 0;
  }
  int d = sol[fst_idx], cur = fst_idx;
  if (d >= n) {
    cout << -1 << endl;
    return 0;
  }
  cout << d - 1 << endl;
  while (d > 0) {
    cout << s[cur] << endl;
    int next = -1;
    REP(i, 0, n) {
      if (dist[cur][i] != 1) { continue; }
      if (sol[i] == d - 1) {
	next = i;
	break;
      }
    }
    assert (next >= 0);
    cur = next;
    d--;
  }
  cout << lst << endl;
}
