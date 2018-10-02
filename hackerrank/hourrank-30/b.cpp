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

const int N = 1e6 + 10;

struct node {
  int fac;
  int v;
  int op;
  node *ch[2];
  node(): fac(1), v(0) {
    ch[0] = ch[1] = 0;
  }
  void dfs(int f = 1) {
    fac = f;
    if (ch[0] == 0) {
      return;
    }
    ch[0]->dfs(f);
    ch[1]->dfs(op * f);
  }
};

node tbl[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  vector<PI> st;
  int n = s.size();
  REP(i, 0, n) {
    tbl[i].fac = 1;
    tbl[i].v = i;
  }
  REP(i, 0, n) {
    if (s[i] == '?') {
      st.push_back(PI(1, i));
    } else {
      st.push_back(PI(2 * i, s[i] == '+' ? 1 : -1));
    }
    while (st.size() >= 3 && st[st.size() - 1].first == 1 && st[st.size() - 2].first == 1) {
      assert (st[st.size() - 3].first % 2 == 0);
      int idx = st[st.size() - 3].first / 2;
      int op = st[st.size() - 3].second;
      int a1 = st[st.size() - 2].second;
      int a2 = st[st.size() - 1].second;
      tbl[idx].ch[0] = &tbl[a1];
      tbl[idx].ch[1] = &tbl[a2];
      tbl[idx].op = op;
      REP(_, 0, 3) st.pop_back();
      st.push_back(PI(1, idx));
    }
  }
  tbl[0].dfs();
  VI pos;
  VI neg;
  REP(i, 0, n) {
    if (s[i] != '?') continue;
    if (tbl[i].fac == 1) pos.push_back(i);
    else neg.push_back(i);
  }
  if (pos.size() < neg.size()) swap(pos, neg);
  VI a(n);
  int ps = pos.size();
  int ng = neg.size();
  REP(i, 0, ps) a[pos[i]] = 1;
  REP(i, 0, ng) {
    a[neg[i]] = ps / ng + (i < ps % ng ? 1 : 0);
  }
  REP(i, 0, n) {
    if (a[i] > 0) cout << a[i] << "\n";
  }
}
