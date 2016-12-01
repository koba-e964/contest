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

int mi = 10000;

void dfs(int x, int v, int vs, int cov, const vector<int> &links) {
  if (v >= x) {
    return;
  }
  if (mi <= __builtin_popcount(vs)) {
    return;
  }
  //cout << "dfs(" << x << "," << v << "," << vs << ")" << endl;
  // cover without v
  if (links[v] != 0) {
    dfs(x, v + 1, vs, cov, links);
  }
  //cover with v
  vs |= 1 << v;
  cov |= 1 << v;
  cov |= links[v];
  if (cov == (1 << x) - 1) {
    mi = min(mi, __builtin_popcount(vs));
    return;
  }
  REP(i, v + 1, x) {
    if (vs & (1 << i)) {
      continue;
    }
    dfs(x, i, vs, cov, links);
    break;
  }
  return;
}


int solve(int x, const vector<PI> &edges, const VI &links) {
  ll es = 0;
  mi = x;
  dfs(x, 0, 0, 0, links);
  return mi;
}


int main(void){
  int x, y;
  while (cin >> x >> y && (x || y)) {
    vector<PI> edges;
    vector<int> link(x);
    REP(i, 0, y) {
      int a, b;
      cin >> a >> b;
      edges.push_back(PI(a, b));
      link[a] |= 1 << b;
      link[b] |= 1 << a;
    }
    cout << solve(x, edges, link) << endl;
  }
}
