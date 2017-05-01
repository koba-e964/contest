#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

typedef long long ll;
typedef vector<int> VI;
typedef pair<ll, ll> PL;
const int N = 3e5 + 10;

#define rep(i, n) for (int i = 0; i <(int)(n); ++i)


ll w[N];
VI edges[N];

vector<ll> sum[N];

ll dfs(int v, int par) {
  ll tot = w[v];
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) { continue; }
    ll res = dfs(w, v);
    sum[v][i] = res;
    tot += res;
  }
  return tot;
}

void dfs_rev(int v, int par, ll passed = 0) {
  ll tot = passed + w[v];
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) {
      sum[v][i] = passed;
      continue;
    }
    tot += sum[v][i];
  }
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) {
      continue;
    }
    dfs_rev(w, v, tot - sum[v][i]);
  }
}
vector<ll> sum_ma[N], sum_mi[N];
pair<ll, ll> dfs_mami(int v, int par) {
  ll tot_ma = w[v];
  ll tot_mi = w[v];
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) { continue; }
    pair<ll, ll> res = dfs_mami(w, v);
    sum_ma[v][i] = res.first;
    sum_mi[v][i] = res.second;
    tot_ma += max(0LL, res.first);
    tot_mi += min(0LL, res.second);
  }
  return make_pair(tot_ma, tot_mi);
}

void dfs_mami_rev(int v, int par, PL passed = PL(0, 0)) {
  PL tot = passed;
  tot.first = max(0LL, passed.first);
  tot.second = min(0LL, passed.second);
  tot.first += w[v];
  tot.second += w[v];
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) {
      sum_ma[v][i] = passed.first;
      sum_mi[v][i] = passed.second;
      continue;
    }
    tot.first += max(0LL, sum_ma[v][i]);
    tot.second += min(0LL, sum_mi[v][i]);
  }
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) {
      continue;
    }
    PL tot_cp(tot);
    tot_cp.first -= max(0LL, sum_ma[v][i]);
    tot_cp.second -= min(0LL, sum_mi[v][i]);
    dfs_mami_rev(w, v, tot_cp);
  }
}

vector<ll> sum_ma_acc[N], sum_mi_acc[N];
pair<ll, ll> dfs_mami_acc(int v, int par) {
  ll tot_ma = 0, tot_mi = 0;
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) { continue; }
    pair<ll, ll> res = dfs_mami_acc(w, v);
    res.first = max(res.first, sum_ma[v][i]);
    res.second = min(res.second, sum_mi[v][i]);
    sum_ma_acc[v][i] = res.first;
    sum_mi_acc[v][i] = res.second;
    tot_ma = max(tot_ma, res.first);
    tot_mi = min(tot_mi, res.second);
  }
  return make_pair(tot_ma, tot_mi);
}

void dfs_mami_acc_rev(int v, int par, PL passed = PL(0, 0)) {
  vector<ll> v_ma, v_mi;
  v_ma.push_back(passed.first);
  v_mi.push_back(passed.second);
  PL par_node;
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) {
      par_node = make_pair(sum_ma[v][i], sum_mi[v][i]);
      sum_ma_acc[v][i] = max(passed.first, par_node.first);
      sum_mi_acc[v][i] = min(passed.second, par_node.second);
      continue;
    }
    v_ma.push_back(sum_ma_acc[v][i]);
    v_mi.push_back(sum_mi_acc[v][i]);
  }
  sort(v_ma.rbegin(), v_ma.rend());
  sort(v_mi.begin(), v_mi.end());
  if (v_ma.size() <= 1) { return; }
  v_ma.erase(v_ma.begin() + 2, v_ma.end());
  v_mi.erase(v_mi.begin() + 2, v_mi.end());
  rep(i, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) {
      continue;
    }
    PL tot_cp(v_ma[0], v_mi[0]);
    if (tot_cp.first == sum_ma_acc[v][i]) {
      tot_cp.first = v_ma[1];
    }
    if (tot_cp.second == sum_mi_acc[v][i]) {
      tot_cp.second = v_mi[1];
    }
    dfs_mami_acc_rev(w, v, tot_cp);
  }
}

VI rev_edge[N];

int main() {
  int n;
  cin >> n;
  for(int w_i = 0; w_i < n; w_i++){
    cin >> w[w_i];
  }
  for(int a0 = 0; a0 < n-1; a0++){
    int u;
    cin >> u;
    int v;
    cin >> v;
    u-- ,v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
    sum[u].push_back(0);
    sum[v].push_back(0);
    sum_ma[u].push_back(0);
    sum_ma[v].push_back(0);
    sum_mi[u].push_back(0);
    sum_mi[v].push_back(0);
    sum_ma_acc[u].push_back(0);
    sum_ma_acc[v].push_back(0);
    sum_mi_acc[u].push_back(0);
    sum_mi_acc[v].push_back(0);
    rev_edge[u].push_back(edges[v].size() - 1);
    rev_edge[v].push_back(edges[u].size() - 1);
  }
  dfs(0, -1);
  dfs_rev(0, -1);
  dfs_mami(0, -1);
  dfs_mami_rev(0, -1);
  dfs_mami_acc(0, -1);
  dfs_mami_acc_rev(0, -1);
  if (0) {
    rep(i, n) {
      rep(j, edges[i].size()) {
	cerr << "e[" << i << "][" << j << "]=" << edges[i][j];
	cerr << ", sum[" << i << "][" << j << "]=" << sum[i][j];
	cerr << endl;
	cerr << "sum_ma[" << i << "][" << j << "]=" << sum_ma[i][j];
	cerr << ", sum_mi[" << i << "][" << j << "]=" << sum_mi[i][j];
	cerr << endl;
	cerr << "sum_ma_acc[" << i << "][" << j << "]=" << sum_ma_acc[i][j];
	cerr << ", sum_mi_acc[" << i << "][" << j << "]=" << sum_mi_acc[i][j];
	cerr << endl;
	cerr << endl;
      }
    }
  }
  ll ma = 0;
  rep(v, n) {
    rep(j, edges[v].size()) {
      int w = edges[v][j];
      int k = rev_edge[v][j];
      ll t1_ma = sum_ma_acc[v][j];
      ll t1_mi = sum_mi_acc[v][j];
      ll t2_ma = sum_ma_acc[w][k];
      ll t2_mi = sum_mi_acc[w][k];
      ma = max(ma, max(t1_ma * t2_ma, t1_mi * t2_mi));
    }
  }
  cout << ma << endl;
  return 0;
}
