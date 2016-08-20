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
typedef pair<int, int> PI;
const double EPS=1e-9;

const ll mod = 1e9 + 7;

const int N = 4000;
ll fact[N];

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll comb(int tot, const VI &ch) {
  ll sum = fact[tot];
  REP(i, 0, ch.size()) {
    sum *= invmod(fact[ch[i]]);
    sum %= mod;
  }
  return sum;
}

void rec(int n, const vector<VI> &edge, vector<ll> &dp, VI& vs, int v, int par) {
  VI ch;
  ll sum = 1;
  vs[v] = 0;
  REP(i, 0, edge[v].size()) {
    int c = edge[v][i];
    if (c == par) continue;
    rec(n, edge, dp, vs, c, v);
    ch.push_back(vs[c] + 1);
    vs[v] += vs[c] + 1;
    sum *= dp[c];
    sum %= mod;
  }
  sum *= comb(vs[v], ch);
  sum %= mod;
  dp[v] = sum;
}

ll solve(int n, const vector<VI> &edge, int a, int b) {
  vector<ll> dp(n);
  VI vs(n);
  rec(n, edge, dp, vs, a, b);
  rec(n, edge, dp, vs, b, a);
  VI t(2);
  t[0] = vs[a];
  t[1] = vs[b];
  ll sum = dp[a] * dp[b] % mod;
  sum *= comb(t[0] + t[1], t);
  sum %= mod;
  return sum;
}


int main(void){
  int n;
  cin >> n;
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  vector<VI> edge(n);
  vector<PI> t;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edge[a].push_back(b);
    edge[b].push_back(a);
    t.push_back(PI(a, b));
  }
  ll sum = 0;
  REP(i, 0, n - 1) {
    int a = t[i].first;
    int b = t[i].second;
    sum += solve(n, edge, a, b);
    sum %= mod;
  }
  cout << sum << endl;
}
