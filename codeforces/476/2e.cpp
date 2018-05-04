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

const int DEBUG = 0;

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

struct node {
  node *ch[26];
  int pop;
  node(): pop(0) {
    REP(i, 0, 26) ch[i] = 0;
  }
  void add(const string &s, int pos) {
    int n = s.length();
    if (pos >= n) {
      this->pop++;
      return;
    }
    int idx = s[pos] - 'a';
    if (ch[idx] == 0) ch[idx] = new node();
    ch[idx]->add(s, pos + 1);
  }
} root;

multiset<int>* dfs(node *t, int dep) {
  vector<pair<int, multiset<int>*> > pool;
  REP(i, 0, 26) {
    if (t->ch[i] == 0) continue;
    multiset<int> *sub = dfs(t->ch[i], dep + 1);
    pool.push_back(make_pair(sub->size(), sub));
  }
  if (t->pop > 0) {
    multiset<int> *ret = new multiset<int>();
    ret->insert(dep);
    pool.push_back(make_pair(1, ret));
  }
  sort(pool.rbegin(), pool.rend());
  // merge
  REP(i, 1, pool.size()) {
    // small to big
    for (int val: *pool[i].second) {
      pool[0].second->insert(val);
    }
    pool[i].second->clear();
    delete pool[i].second;
  }
  multiset<int> *ret = pool[0].second;
  if (dep > 0) {
    assert (ret->size() > 0);
    auto it = ret->end();
    it--;
    if (ret->count(dep) == 0) {
      ret->erase(it);
      ret->insert(dep);
    }
  }
  if (DEBUG) {
    cerr << "dfs " << dep << endl;
    for (auto i: *ret) cerr << " " << i;
    cerr << endl;
  }
  return ret;
}


// The author implemented the solution after reading the editorial.
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin>>n;
  vector<string> s(n);
  REP(i, 0, n) cin >> s[i];
  ll tot = 0;
  REP(i, 0, n) root.add(s[i], 0);
  multiset<int> *res = dfs(&root, 0);
  for (int val: *res) tot += val; 
  cout << tot << endl;
}
