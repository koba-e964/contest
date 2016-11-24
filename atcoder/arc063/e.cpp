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
const int DEBUG = 1;

const int N = 100100;
int a[N], b[N], v[N], p[N];

VI edges[N];
PI cons[N];
int val[N];
int col[N];

PI intersec(PI a, PI b) {
  if (a.first > a.second) {
    return PI(0, -1);
  }
  if (b.first > b.second) {
    return PI(0, -1);
  }
  if ((a.first - b.first) % 2 != 0 || a.second < b.first || b.second < a.first) {
    return PI(0, -1);
  }
  return PI(max(a.first, b.first), min(a.second, b.second));
}

PI dfs(int v, int par, PI c) {
  if (c.first > c.second) {
    return PI(0, -1);
  }
  //assert ((c.second - c.first) % 2 == 0);
  if (val[v] >= 0) {
    c = intersec(c, PI(val[v], val[v]));
  }
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) {
      continue;
    }
    PI child_c = dfs(w, v, PI(c.first - 1, c.second + 1));
    if (child_c.first > child_c.second) {
      return cons[v]=PI(0, -1);
    }
    child_c.first--;
    child_c.second++;
    c = intersec(c, child_c);
  }
  cons[v] = c;
  return c;
}

void dfs2(int v, int par, int c) {
  //assert (cons[v].first <= c && c <= cons[v].second);
  col[v] = c;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) {
      continue;
    }
    if (cons[w].first <= c - 1 && c - 1 <= cons[w].second) {
      dfs2(w, v, c - 1);
    } else {
      dfs2(w, v, c + 1);
    }
  }
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    cin >> a[i] >> b[i];
    a[i]--, b[i]--;
    edges[a[i]].push_back(b[i]);
    edges[b[i]].push_back(a[i]);
  }
  int k;
  cin >> k;
  int root = -1;
  REP(i, 0, N) {
    val[i] = -1;
  }
  REP(i, 0, k) {
    cin >> v[i] >> p[i];
    v[i]--;
    if (root == -1) {
      root = v[i];
    }
    val[v[i]] = p[i];
  }
  assert (root >= 0);
  REP(i, 0, N) {
    cons[i] = PI(-1, -1);
  }
  PI ok = dfs(root, -1, PI(val[root], val[root]));
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "cons["  << i << "]=" << cons[i].first << "," << cons[i].second << endl;
    }
  }
  if (ok.first > ok.second) {
    cout << "No" << endl;
    return 0;
  }
  cout << "Yes" << endl;
  dfs2(root, -1, val[root]);
  REP(i, 0, n) {
    cout << col[i] << endl;
  }
}
