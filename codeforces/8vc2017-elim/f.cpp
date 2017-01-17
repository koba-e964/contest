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

/**
 * Strong connected components.
 * Header requirement: algorithm, cassert, set, vector
 * Verified by: AtCoder ARC010 (http://arc010.contest.atcoder.jp/submissions/1015294)
 *              yukicoder No. 19 (http://yukicoder.me/submissions/141513)
 */
class SCC {
private:
  int n;
  int ncc;
  typedef std::vector<int> vi;
  std::vector<vi> g; // graph in adjacent list
  std::vector<vi> rg; // reverse graph
  vi vs;
  std::vector<bool> used;
  vi cmp;
public:
  SCC(int n): n(n), ncc(-1), g(n), rg(n), vs(n), used(n), cmp(n) {}
  void add_edge(int from, int to) {
    g[from].push_back(to);
    rg[to].push_back(from);
  }
private:
  void dfs(int v) {
    used[v] = true;
    for (int i = 0; i < g[v].size(); ++i) {
      if (!used[g[v][i]]) { dfs(g[v][i]); }
    }
    vs.push_back(v);
  }
  void rdfs(int v, int k) {
    used[v] = true;
    cmp[v] = k;
    for (int i = 0; i < rg[v].size(); ++i) {
      if (!used[rg[v][i]]) { rdfs(rg[v][i], k); }
    }
  }
public:
  int scc() {
    std::fill(used.begin(), used.end(), 0);
    vs.clear();
    for (int v = 0; v < n; ++v) {
      if (!used[v]) { dfs(v); }
    }
    std::fill(used.begin(), used.end(), 0);
    int k = 0;
    for (int i = vs.size() - 1; i >= 0; --i) {
      if (!used[vs[i]]) { rdfs(vs[i], k++); }
    }
    return ncc = k;
  }
  std::vector<int> top_order() const {
    if (ncc == -1) assert(0);
    return cmp;
  }
  std::vector<std::vector<int> > scc_components(void) const {
    if (ncc == -1) assert(0);
    std::vector<std::vector<int> > ret(ncc);
    for (int i = 0; i < n; ++i) {
      ret[cmp[i]].push_back(i);
    }
    return ret;
  }
  /*
   * Returns a dag whose vertices are scc's, and whose edges are those of the original graph, in the adjacent-list format.
   */
  std::vector<std::vector<int> > dag() const {
    if (ncc == -1) {
      assert(0);
    }
    typedef std::set<int> si;
    std::vector<si> ret(ncc);
    for (int i = 0; i < g.size(); ++i) {
      for (int j = 0; j < g[i].size(); ++j) {
	int to = g[i][j];
	if (cmp[i] != cmp[to]) {
	  assert (cmp[i] < cmp[to]);
	  ret[cmp[i]].insert(cmp[to]);
	}
      }
    }
    std::vector<std::vector<int> > vret(ncc);
    for (int i = 0; i < ncc; ++i) {
      vret[i] = std::vector<int>(ret[i].begin(), ret[i].end());
    }
    return vret;
  }
  std::vector<std::vector<int> > rdag() const {
    if (ncc == -1) {
      assert(0);
    }
    typedef std::set<int> si;
    std::vector<si> ret(ncc);
    for (int i = 0; i < g.size(); ++i) {
      for (int j = 0; j < g[i].size(); ++j) {
	int to = g[i][j];
	if (cmp[i] != cmp[to]) {
	  assert (cmp[i] < cmp[to]);
	  ret[cmp[to]].insert(cmp[i]);
	}
      }
    }
    std::vector<std::vector<int> > vret(ncc);
    for (int i = 0; i < ncc; ++i) {
      vret[i] = std::vector<int>(ret[i].begin(), ret[i].end());
    }
    return vret;
  }
};

const int K = 1000100 / 32 + 1;
const int T = 100;
int dp1[2][K];
int c[32 * K];


/* I implemented this code after reading the editorial
 *   (http://codeforces.com/blog/entry/49793)
 */
int main(void){
  ios::sync_with_stdio(false);
  int n, k;
  cin >> n >> k;
  VI p(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }
  SCC scc(n);
  REP(i, 0, n) {
    scc.add_edge(i, p[i]);
  }
  int ncc = scc.scc();
  vector<VI> comp = scc.scc_components();
  int others;
  REP(i, 0, ncc) {
    assert (comp[i].size() >= 2);
  }
  // max
  int ma = 0;
  {
    VI t(ncc);
    const int BIAS = 1e7;
    REP(i, 0, ncc) {
      int s = comp[i].size();
      t[i] = s % 2 == 0 ? - BIAS - s : -s;
    }
    sort(t.begin(), t.end());
    int rem = k;
    int residue = 0;
    REP(i, 0, ncc) {
      int s = t[i] % 2 == 0 ? -BIAS - t[i] : -t[i];
      int u = min(rem, s / 2);
      rem -= u;
      ma += min(s, 2 * u);
      if (s % 2 == 1) { residue++; }
    }
    ma += min(residue, rem);
  }
  // min
  int mi = 0;
  {
    VI giant;
    VI baby(T, 0);
    REP(i, 0, ncc) {
      int tt = comp[i].size();
      if (tt >= T) {
	giant.push_back(tt);
      } else {
	baby[tt]++;
      }
    }
    // giant
    // Use bit map
    const int B = 32;
    int w = giant.size();
    dp1[0][0] = 1;
    REP(i, 0, w) {
      int t = i % 2;
      REP(j, 0, K) {
	dp1[1 - t][j] = dp1[t][j];
      }
      int bitsh = giant[i] % B;
      int wordsh = giant[i] / B;
      REP(j, wordsh, K) {
	dp1[1 - t][j] |= dp1[t][j - wordsh] << bitsh;
	if (j >= wordsh + 1 && bitsh > 0) {
	  dp1[1 - t][j] |= (unsigned int)dp1[t][j - wordsh - 1] >> (32 - bitsh);
	}
      }
    }
    // baby
    c[0] = 0;
    REP(i, 1, k + 1) {
      c[i] = -1;
    }
    REP(i, 2, T) {
      int mx = baby[i];
      REP(j, 0, k + 1) {
	c[j] = c[j] < 0 ? -1 : 0;
	if (j < i) { continue; }
	if (c[j] >= 0) { continue; }
	if (c[j - i] < 0) { continue; }
	c[j] = c[j - i] + 1;
	if (c[j] > mx) c[j] = -1;
      }
    }
    if (0) {
      cerr << "c:";
      REP(i, 0, k + 1) {
	cerr << " " << c[i];
      }
      cerr << endl;
    }
    bool ok = false;
    REP(i, 0, k + 1) {
      ok |= (dp1[w % 2][i / B] & (1 << (i % B))) != 0 && c[k - i] >= 0;
      if (ok) break;
    }
    mi = ok ? k : k + 1;
  }
  cout << min(mi, n) << " " << ma << "\n";
}
