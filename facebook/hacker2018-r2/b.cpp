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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

set<int> *merge(set<int> *a, set<int> *b) {
  if (a->size() < b->size()) swap(a, b);
  for (int v: *b) {
    a->insert(v);
  }
  delete b;
  return a;
}

struct cmp {
  bool operator()(pair<int, set<int>*> x, pair<int, set<int>*> y) const {
    return x.first < y.first;
  }
};

const int N = 200100;

VI ch[N];
vector<set<int>*> node;
VI freq;

ll dfs(int i) {
  assert (node.size() == freq.size());
  ll tot = 0;
  REP(j, 0, ch[i].size()) {
    assert (ch[i][j] < node.size());
    assert (ch[i][j] > 0);
    tot += dfs(ch[i][j]);
  }
  if (DEBUG) {
    REP(j, 0, node.size()) DEBUGP(node[j]);
  }
  vector<pair<int, set<int>*> > pool;
  pool.push_back(make_pair(node[i]->size(), node[i]));
  node[i] = NULL;
  REP(j, 0, ch[i].size()) {
    int v = ch[i][j];
    assert (v >= 0 && v < node.size());
    assert (node[v]);
    pool.push_back(make_pair(node[v]->size(), node[v]));
    node[v] = NULL;
  }
  //sort(pool.rbegin(), pool.rend(), cmp());
  assert (pool.size() >= 1);
  set<int> *princ = pool[0].second;
  pool[0].second = NULL;
  REP(j, 1, pool.size()) {
    princ = merge(princ, pool[j].second);
    pool[j].second = NULL;
  }
  pool.clear();
  REP(j, 0, freq[i]) {
    if (not princ->empty()) {
      assert (princ->size() > 0);
      int val = *princ->rbegin();
      tot += val;
      princ->erase(val);
    } else {
      break;
    }
  }
  node[i] = princ;
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    cout << "Case #" << case_nr << ": ";
    int n, m;
    ll a, b;
    cin >> n >> m >> a >> b;
    VI p(n, -1);
    REP(i, 1, n) {
      cin >> p[i];
      ch[p[i]].push_back(i);
    }
    VI c(m);
    REP(i, 0, m) c[i] = (a * i + b) % n;
    REP(i, 0, m) assert (c[i] >= 0 && c[i] < n);
    if (DEBUG) {
      cerr << "c:";
      REP(i, 0, m) cerr << " " << c[i];
      cerr << endl;
    }
    freq = VI(n, 0);
    REP(i, 0, m) freq[c[i]] += 1;
    if (DEBUG) {
      cerr << "freq:";
      REP(i, 0, n) cerr << " " << freq[i];
      cerr << endl;
    }
    node = vector<set<int>*>(n);
    REP(i, 0, n) {
      node[i] = new set<int>();
      assert (node[i]);
      node[i]->insert(i);
    }
    cerr << "doing...";
    ll tot = dfs(0);
    cerr << "done" << endl;
    cout << tot << endl;
    // cleanup
    REP(i, 0, n) {
      ch[i].clear();
    }
    node.clear();
    freq.clear();
  }
}
