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

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};


const int N = 5000;
const int D = 200;
double pt[N][D];
double len[N];

ll hash_v[N][3];

// Took a look at the editorial
int main(void){
  ll seed;
  cin >> seed;
  unsigned x = 123456789;
  unsigned y = 362436069;
  unsigned z = 521288629;
  unsigned w = seed;

  REP(i, 0, N) {
    double sum = 0;
    REP(j, 0, D) {
      unsigned t = x ^ (x << 11);
      x = y;
      y = z;
      z = w;
      w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));
	
      int v = w % 100000 - 50000;
      if (v >= 0) {
	v = v + 1;
      }
      pt[i][j] = v;
      sum += double(v) * v;
      hash_v[i][j / 64] |= v > 0 ? 0 : 1LL << (j % 64);
    }
    len[i] = sqrt(sum);
  }
  typedef pair<int, PI> PIPI;
  vector<PIPI> edges;
  REP(i, 0, N) {
    REP(j, i + 1, N) {
      int dist = __builtin_popcountll(hash_v[i][0] ^ hash_v[j][0])
	+ __builtin_popcountll(hash_v[i][1] ^ hash_v[j][1])
	+ __builtin_popcountll(hash_v[i][2] ^ hash_v[j][2]);
      edges.push_back(PIPI(dist, PI(i, j)));
    }
  }
  sort(edges.begin(), edges.end());
  int limit = 100 * N;
  //int limit = N * (N - 1) / 2;
  vector<pair<double, PI> > refined(limit);
  REP(i, 0, limit) {
    PIPI e = edges[i];
    int u = e.second.first;
    int v = e.second.second;
    double dist = len[u] * len[v];
    double inn = 0;
    REP(j, 0, D) {
      inn += pt[u][j] * pt[v][j];
    }
    dist = inn / dist;
    refined[i] = make_pair(1 - dist, PI(u, v));
  }
  sort(refined.begin(), refined.end());
  UnionFind uf(N);
  double tot = 0;
  int cnt = 0;
  REP(i, 0, refined.size()) {
    pair<double, PI> e = refined[i];
    int u = e.second.first;
    int v = e.second.second;
    if (not uf.is_same_set(u, v)) {
      cout << u + 1 << " " << v + 1 << endl;
      uf.unite(u, v);
      tot += e.first;
      cnt++;
    }
  }
  //cout << "count = " << cnt << endl;
  //cout << "total = " << tot << endl;
}
