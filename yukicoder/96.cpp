#include <algorithm>
#include <iostream>
#include <cstdio>
#include <cmath>
#include <vector>

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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
const double EPS=1e-9;

const int N = 120001;
int x[N], y[N];

const int W = 2100;
const int B = 10;
const int TH = 200;
VI board[W][W];


ll solve(const vector<int> &t) {
  int n = t.size();
  ll ma = 0;
  if (n <= TH) {
    REP(j, 0, n) {
      REP(k, j + 1, n) {
	int a = t[j];
	int b = t[k];
	ll dist = (x[a] - x[b]) * (x[a] - x[b]) + (y[a] - y[b]) * (y[a] - y[b]);
	ma = max(ma, dist);
      }
    }
    return ma;
  }
  // rotation
  REP(i, 0, TH) {
    double pi = acos(-1.0);
    double s = sin(pi * i / TH);
    double c = cos(pi * i / TH);
    typedef pair<double, int> PDI;
    vector<PDI> ss(n);
    REP(i, 0, n) {
      ss[i] = PDI(c * x[t[i]] + s * y[t[i]], t[i]);
    }
    sort(ss.begin(), ss.end());
    int u = ss[0].second;
    int v = ss[ss.size() - 1].second;
    ma = max(ma, ll(x[u] - x[v]) * (x[u] - x[v]) + (y[u] - y[v]) * (y[u] - y[v]));
  }
  return ma;
}

int main(void){
  int n;
  cin >> n;
  if (n == 0) {
    cout << 1 << endl;
    return 0;
  }
  const int BIAS = 10000;
  UnionFind uf(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
    x[i] += BIAS;
    y[i] += BIAS;
    board[x[i] / B][y[i] / B].push_back(i);
  }
  REP(i, 0, n) {
    REP(a, -1, 2) {
      REP(b, -1, 2) {
	if (x[i] / B + a < 0 || y[i] / B + b < 0) { continue; }
	VI &neighbor = board[x[i] / B + a][y[i] / B + b];
	REP(k, 0, neighbor.size()) {
	  int j = neighbor[k];
	  int dist = (x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j]);
	  if (dist <= 100) {
	    uf.unite(i, j);
	  }
	}
      }
    }
  }
  ll ma = 0;
  vector<VI> conn(n);
  REP(i, 0, n) {
    conn[uf.root(i)].push_back(i);
  }
  REP(i, 0, n) {
    VI &t = conn[i];
    ma = max(ma, solve(t));
  }
  printf("%.15f\n", 2 + sqrt(ma));
}
