#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

struct Hash {
  ll h[2];
  static const ll md[2];
  Hash(ll v) {
    REP(i, 0, 2) { h[i] = ((v % md[i]) + md[i]) % md[i]; }
  }
  Hash() {}
  Hash operator+(Hash other) const {
    Hash ret;
    REP(i, 0, 2) {
      ret.h[i] = (h[i] + other.h[i]) % md[i];
    }
    return ret;
  }
  Hash operator*(Hash other) const {
    Hash ret;
    REP(i, 0, 2) {
      ret.h[i] = (h[i] * other.h[i]) % md[i];
    }
    return ret;
  }
  bool operator<(Hash other) const {
    if (h[0] != other.h[0]) {
      return h[0] < other.h[0];
    }
    return h[1] < other.h[1];
  }
};
const ll Hash::md[2] = {(ll)1e9 + 7, (ll)1e9 + 9};


const int N = 123456;
VI edges[N];
VI rev[N];
vector<Hash> dp[N];
Hash dp_round[N];

Hash dfs(int v, int p = -1) {
  vector<Hash> children;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == p) { continue; }
    Hash h = dfs(w, v);
    dp[v][i] = h;
    children.push_back(h);
  }
  // compute hash by children
  Hash ret(13579LL);
  for (Hash h: children) {
    ret = ret + h * h;
  }
  return ret;
}

void dfs_remain(int v, int p, Hash r) {
  REP(i, 0, edges[v].size()) {
    if (edges[v][i] == p) {
      dp[v][i] = r;
      break;
    }
  }
  Hash acc(13579LL);
  REP(i, 0, edges[v].size()) {
    acc = acc + dp[v][i] * dp[v][i];
  }
  dp_round[v] = acc;
  
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == p) { continue; }
    Hash subt = acc + dp[v][i] * dp[v][i] * (-1);
    dfs_remain(w, v, subt);
  }
}

void add(map<Hash, int> &m, Hash h, int v) {
  if (m.count(h)) {
    int r = m[h];
    if (r + v == 0) {
      m.erase(h);
    } else {
      m[h] = r + v;
    }
  } else {
    m[h] = v;
  }
}

// collect subtrees' information
void dfs_collect(int v, int p, map<Hash, int> &m) {
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == p) { continue; }
    add(m, dp[v][i], 1);
    dfs_collect(w, v, m);
  }
}

int ma;
int maxi;

void dfs_exchange(int v, int p, map<Hash, int> &m) {
  if (ma < m.size()) {
    ma = m.size();
    maxi = v;
  }
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == p) { continue; }
    // move to w, add dp[w][rev[v][i]] from m and erase dp[v][i] to m
    add(m, dp[v][i], -1);
    add(m, dp[w][rev[v][i]], 1);
    dfs_exchange(w, v, m);
    add(m, dp[v][i], 1);
    add(m, dp[w][rev[v][i]], -1);
  }
}

/**
 * Reference: http://codeforces.com/contest/763/submission/24384757
 *              by uwi
 */
int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
    rev[a].push_back(edges[b].size() - 1);
    rev[b].push_back(edges[a].size() - 1);
    dp[a].push_back(Hash());
    dp[b].push_back(Hash());
  }
  dfs(0);
  dfs_remain(0, -1, Hash());
  if (0) {
    REP(i, 0, n) {
      cerr << "dp[" << i << "]:";
      REP(j, 0, dp[i].size()) {
	cerr << " " << dp[i][j].h[0];
      }
      cerr << endl;
    }
  }
  map<Hash, int> m;
  dfs_collect(0, -1, m);
  ma = m.size();
  maxi = 0;
  dfs_exchange(0, -1, m);
  cout << maxi + 1 << endl;
}
