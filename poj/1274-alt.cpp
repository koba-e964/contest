#include <vector>
/**
 * Bipartite Matching with a simple dfs.
 * Header requirement: vector
 * Verified by: POJ 1274 (http://poj.org/problem?id=1742)
 */
class BipartiteMatching {
private:
  std::vector<std::vector<int> > graph; // [v1][?]
  std::vector<int> match; // [v2]
  std::vector<bool> used; // [v1]
public:
  /* v is the number of vertices (labeled from 0 .. v-1) */
  BipartiteMatching(int v1, int v2) : graph(v1), match(v2), used(v1) {}
  void add_edge(int from, int to) {
    graph[from].push_back(to);
  }
  bool dfs(int v) {
    used[v] = true;
    for (int i = 0; i < graph[v].size(); ++i) {
      int u = graph[v][i];
      int w = match[u];
      if (w < 0 || (!used[w] && dfs(w))) {
	match[u] = v;
	return true;
      }
    }
    return false;
  }
  int matching(void) {
    int res = 0;
    std::fill(match.begin(), match.end(), -1);
    int n = match.size();
    for (int v = 0; v < graph.size(); ++v) {
      std::fill(used.begin(), used.end(), false);
      if (dfs(v)) {
	res++;
      }
    }
    return res;
  }
};
#include <algorithm>
#include <cstdio>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

int main(void){
  int n, m;
  for(;scanf("%d%d",&n,&m)>=2;){
    BipartiteMatching bm(n, m);
    REP(i, 0, n) {
      int si;
      scanf("%d",&si);
      REP(j, 0, si) {
	int s;
	scanf("%d",&s);
	bm.add_edge(i, s - 1);
      }
    }
    printf("%d\n",bm.matching());
  }
}
