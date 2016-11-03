#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll inf = 1e16;

typedef pair<ll, int> PLI;
const int N = 100100;
vector<PLI> edges[N];


// n: #vertices, edges: edges in adjacent-list format
ll prim(int n) {
  vector<bool> used(n, false);
  priority_queue<PLI, vector<PLI>, greater<PLI> > que;
  que.push(PLI(0, 0));
  ll res = 0;
  for (;not que.empty();) {
    PLI t = que.top(); que.pop();
    int v = t.second;
    int c = t.first;
    if (used[v]) {
      continue;
    }
    res += c;
    used[v] = true;
    REP(j, 0, edges[v].size()) {
      PLI e = edges[v][j];
      if (not used[e.second]) {
	que.push(e);
      }
    }
  }
  return res;
}


int main(void){
  ios::sync_with_stdio(0);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    ll c;
    cin >> c;
    edges[n].push_back(PLI(c, i));
    edges[i].push_back(PLI(c, n));
  }
  REP(i, 0, m) {
    int a, b, r;
    cin >> a >> b >> r;
    a--, b--;
    edges[a].push_back(PLI(r, b));
    edges[b].push_back(PLI(r, a));
  }
  cout << prim(n + 1) << "\n";
}
