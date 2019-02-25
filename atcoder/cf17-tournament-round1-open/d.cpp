#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int n;
const int N = 110000;
VI edges[N];
int p[N];
ll s[N];
int size[N];
ll wei[N];
int sa, sb;

int dfs(int v, int par) {
  p[v] = par;
  int sz = 1;
  for (int w: edges[v]) {
    if (w == par) continue;
    int sub = dfs(w, v);
    if (2 * sub != n) {
      wei[w] = (s[v] - s[w]) / (2 * sub - n);
      cerr << w << " " << wei[w] << endl;
    } else {
      sa = v;
      sb = w;
    }
    sz += sub;
  }
  return size[v] = sz;
}

ll dfs2(int v, int par) {
  ll sum = 0;
  for (int w: edges[v]) {
    if (w == par) continue;
    ll sub = dfs2(w, v);
    sum += sub + size[w] * wei[w];
  }
  return sum;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  sa = -1, sb = -1;
  vector<PI> e(n - 1);
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
    e[i] = PI(a, b);
  }
  REP(i, 0, n) {
    cin >> s[i];
  }
  VL r(n - 1);
  dfs(0, -1);
  if (sa != -1 || sb != -1) {
    ll sum = dfs2(0, -1);
    wei[sb] = (s[0] - sum) / (n / 2);
  }
  REP(i, 0, n - 1) {
    int a = e[i].first, b = e[i].second;
    if (p[a] == b) {
      r[i] = wei[a];
    } else {
      r[i] = wei[b];
    }
  }
  REP(i, 0, n - 1) {
    cout << r[i] << endl;
  }
}
