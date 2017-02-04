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

const int N = 123456;
VI edges[N];
ll c[N];

const ll inf = 1e14;
bool ok = true;
ll dfs(int v, int p) {
  int idx = edges[v].size();
  if (idx == 1 && p >= 0) {
    return c[v];
  }
  if (idx == 1) {
    ll q = dfs(edges[v][0], v);
    ok &= q == c[v];
    return q;
  }
  /*
  if (idx == 2) {
    int w1 = edges[v][0];
    int w = edges[v][1];
    return c[v] <= c[w1] && c[v] <= c[w];
    && dfs(w, v) && d;
  }
  */
  ll tot = 0;
  VL qs;
  for (auto w: edges[v]) {
    if (w == p) { continue; }
    ll q = dfs(w, v);
    //cerr << v << "==>" << w << " " << q << endl;
    tot += q;
    qs.push_back(q);
  }
  assert (qs.size() >= 1);
  ok &= tot <= 2 * c[v];
  ll ret = 2 * c[v] - tot;
  qs.push_back(ret);
  for (auto q: qs) {
    if (q > c[v]) {
      ok = false;
    }
  }
  return ret;
  
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> c[i];
  }
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  int l = -1;
  REP(i, 0, n) {
    if (edges[i].size() == 1) {
      l = i;
      break;
    }
  }
  dfs(l, -1);
  cout << (ok ? "YES" : "NO") << endl;
}
