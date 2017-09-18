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

map<string, int> pool;
vector<string> inv_pool;
int get(const string &s) {
  if (pool.count(s)) {
    return pool[s];
  }
  int sz = pool.size();
  pool[s] = sz;
  inv_pool.push_back(s);
  return sz;
}

string to_s(int n) {
  stringstream ss;
  ss << n;
  return ss.str();
}

void add_moves(vector<PI> &mvs, const set<int> &indices,
	       const vector<string> &dat) {
  VI disapp;
  set<int> unapp(indices);
  for (const string &s: dat) {
    int idx = get(s);
    if (indices.count(idx)) {
      unapp.erase(idx);
      continue; // no move necessary
    }
    disapp.push_back(idx);
  }
  assert (disapp.size() == unapp.size());
  VI unapp_v(unapp.begin(), unapp.end());
  REP(i, 0, disapp.size()) {
    mvs.push_back(PI(disapp[i], unapp_v[i]));
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<string> ex; // should come first;
  vector<string> reg;
  vector<string> name(n);
  REP(i, 0, n) {
    int ty;
    cin >> name[i] >> ty;
    if (ty == 1) {
      ex.push_back(name[i]);
    } else {
      reg.push_back(name[i]);
    }
    get(name[i]);
  }
  REP(i, 1, n + 1) {
    (void) get(to_s(i));
  }
  set<int> exidx;
  set<int> regidx;
  REP(i, 1, ex.size() + 1) {
    exidx.insert(get(to_s(i)));
  }
  REP(i, ex.size() + 1, n + 1) {
    regidx.insert(get(to_s(i)));
  }
  // compare collectively,
  vector<PI> mvs;
  add_moves(mvs, exidx, ex);
  add_moves(mvs, regidx, reg);
  if (0) {
    cerr << "moves:"<<endl;
    REP(i, 0, mvs.size()) {
      int u = mvs[i].first;
      int v = mvs[i].second;
      cerr << "mv " << u << "(" << inv_pool[u] << ") "
	   << v << "(" << inv_pool[v] << ")" << endl;
    }
  }
  int sz = pool.size();
  vector<VI> edges(sz);
  vector<VI> rev(sz);
  VI deg(sz);
  REP(i, 0, mvs.size()) {
    int u = mvs[i].first;
    int v = mvs[i].second;
    edges[u].push_back(v);
    rev[v].push_back(u);
    deg[u] += 1;
  }
  set<PI> mv_set(mvs.begin(), mvs.end());
  // generate dummy string;
  mt19937 rand(0xe869120);
  string dummy;
  string alpha = "0123456789abcdefghijklmnopqrstuvwxyz";
  assert (alpha.length() == 36);
  while (true) {
    string s;
    REP(i, 0, 6) {
      int r = rand() % 36;
      s += alpha[r];
    }
    if (pool.count(s) == 0) {
      dummy = s;
      break;
    }
  }
  if (0) {
    cerr << "dummy = " << dummy << endl;
  }
  vector<pair<string, string> > ans;
  queue<int> que;
  REP(i, 0, sz) {
    if (deg[i] == 0) {
      if (rev[i].size() > 0) {
	que.push(i);
      }
    }
  }
  while (not que.empty()) {
    int t = que.front(); que.pop();
    if (rev[t].empty()) { continue; }
    int u = rev[t][0];
    // u --> t
    ans.push_back(make_pair(inv_pool[u], inv_pool[t]));
    assert (mv_set.count(PI(u, t)));
    mv_set.erase(PI(u, t));
    que.push(u);
  }
  for (auto mv: mv_set) {
    int u = mv.first;
    int v = mv.second;
    if (mv_set.count(PI(v, u))) {
      if (u < v) {
	ans.push_back(make_pair(inv_pool[u], dummy));
	ans.push_back(make_pair(inv_pool[v], inv_pool[u]));
	ans.push_back(make_pair(dummy, inv_pool[v]));
      }
      continue;
    }
    assert (0);
  }
  cout << ans.size() << "\n";
  REP(i, 0, ans.size()) {
    cout << "move " << ans[i].first << " " << ans[i].second << "\n";
  }
}
