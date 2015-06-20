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

const int N = 1001;
int x[N], y[N];

int main(void){
  int n;
  cin >> n;
  if (n == 0) {
    cout << 1 << endl;
    return 0;
  }
  UnionFind uf(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      int dist = (x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j]);
      if (dist <= 100) {
	uf.unite(i, j);
      }
    }
  }
  double ma = 0;
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (uf.is_same_set(i, j)) {
	double dist = (x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j]);
	ma = max(ma, dist);
      }
    }
  }
  printf("%.9f\n", 2 + sqrt(ma));
}
