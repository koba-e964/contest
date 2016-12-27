#include <algorithm>
#include <iostream>
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

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int W = 1001;
ll pow_memo[W][W];
ll inv_memo[W];


ll count_surj(int n, int k) {
  if (n < k) {
    return 0;
  }
  ll sum = 0;
  ll comb = 1;
  REP(i, 0, k) {
    sum += (pow_memo[k - i][n] * comb % mod) * (i % 2 == 0 ? 1 : mod - 1);
    sum %= mod;
    comb = comb * (k - i) % mod;
    comb = comb * inv_memo[1 + i] % mod;
  }
  return sum;
}

int main(void){
  int n, m;
  cin >> n >> m;
  VI w(n);
  REP(i, 0, n) {
    cin >> w[i];
  }
  vector<PI> e;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--;
    v--;
    e.push_back(PI(u, v));
  }
  REP(i, 1, W) {
    REP(j, 0, W) {
      pow_memo[i][j] = powmod(i, j);
    }
    inv_memo[i] = powmod(i, mod - 2);
  }
  // O(nm)-time
  ll tot = 0;
  REP(i, 0, n) {
    // retain A_k s.t. w_k >= w_i
    vector<VI> edges(n);
    for (PI uv : e) {
      int u = uv.first;
      int v = uv.second;
      if (w[u] >= w[i] && w[v] >= w[i]) {
	edges[v].push_back(u);
      }
    }
    // Vertices reachable to i
    VI reach(n, false);
    queue<int> que;
    que.push(i);
    while (not que.empty()) {
      int v = que.front(); que.pop();
      if (reach[v]) { continue; }
      reach[v] = true;
      REP(j, 0, edges[v].size()) {
	int w = edges[v][j];
	if (not reach[w]) {
	  que.push(w);
	}
      }
    }
    REP(j, 0, n) {
      if (reach[j]) {
	tot += count_surj(w[j], w[i]);
	tot %= mod;
      }
    }
  }
  cout << tot << endl;
}
