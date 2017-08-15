#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

class OwaskiAndTree {
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
  typedef long long int ll;
  typedef vector<int> VI;
  typedef pair<ll, ll> PL;
  vector<ll> pl;
  vector<VI> edges;
  // (maximum pl, maximum pleasure if reset)
  PL dfs(int v) {
    ll fst = pl[v];
    ll snd = 0;
    vector<PL> pool;
    REP(i, 0, edges[v].size()) {
      int w = edges[v][i];
      PL sub = dfs(w);
      fst += max(0LL, sub.first);
      snd += max(sub.first, sub.second);
    }
    return PL(fst, snd);
  }
public:
  int maximalScore(vector <int> parent, vector <int> pleasure) {
    int n = pleasure.size();
    pl = vector<ll>(n);
    REP(i, 0, n) {
      pl[i] = pleasure[i];
    }
    edges = vector<VI>(n);
    REP(i, 0, n - 1) {
      edges[parent[i]].push_back(i + 1);
    }
    PL res = dfs(0);
    return max(res.first, res.second);
  }
};
