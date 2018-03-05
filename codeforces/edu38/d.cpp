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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;
typedef pair<ll, int> PLI;
const ll mod = 1e9 + 7;

const int N = 200100;
const ll inf = 1e18;

vector<PIL> edges[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int v, u;
    ll w;
    cin >> v >> u >> w;
    v--, u--;
    edges[v].push_back(PIL(u, w));
    edges[u].push_back(PIL(v, w));
  }
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL cost(n, inf);
  priority_queue<PLI, vector<PLI>, greater<PLI> > que;
  REP(i, 0, n) {
    que.push(PLI(a[i], i));
  }
  while (not que.empty()) {
    PLI top = que.top(); que.pop();
    ll cs = top.first;
    int v = top.second;
    if (cost[v] <= cs) continue;
    cost[v] = cs;
    REP(i, 0, edges[v].size()) {
      int w = edges[v][i].first;
      ll nc = cs + 2 * edges[v][i].second;
      if (cost[w] <= nc) continue;
      que.push(PLI(nc, w));
      cost[w] = nc + 1;
    }
  }
  REP(i, 0, n) {
    cout << cost[i] << (i == n - 1 ? "\n" : " ");
  }
}
