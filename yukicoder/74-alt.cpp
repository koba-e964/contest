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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 110;
bool odd[N] = {};
int ww[N] = {};
int acc[N] = {};
int main(void){
  int n;
  cin >> n;
  UnionFind uf(n);
  REP(i, 0, n) {
    int d;
    cin >> d;
    int r1 = (i + d) % n;
    int r2 = (i - d % n + n) % n;
    if (r1 == r2) {
      odd[r1] = 1;
    } else {
      uf.unite(r1, r2);
    }
  }
  REP(i, 0, n) {
    int w;
    cin >> w;
    ww[i] = 1 - w;
  }
  REP(i, 0, n) {
    odd[uf.root(i)] |= odd[i];
  }
  REP(i, 0, n) {
    acc[uf.root(i)] += ww[i];
  }
  REP(i, 0, n) {
    if (!odd[i] && acc[i] % 2) {
      cout << "No" << endl;
      return 0;
    }
  }
  cout << "Yes" << endl;
  return 0;
}
