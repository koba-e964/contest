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

PI sigma(PI x, PI y) {
  set<int> u;
  u.insert(x.first);
  u.insert(x.second);
  int cnt = 0;
  cnt += u.count(y.first);
  cnt += u.count(y.second);
  int com = -1;
  if (cnt == 1) {
    com = u.count(y.first) ? y.first : y.second;
  }
  return PI(cnt, com);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<PI> iroha(n), resha(m);
  REP(i, 0, n) {
    int x, y;
    cin >> x >> y;
    iroha[i] = PI(x, y);
  }
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    resha[i] = PI(x, y);
  }
  set<int> p, q;
  int jinsei = 0;
  REP(i, 0, n) {
    set<int> cod;
    REP(j, 0, m) {
      PI tapris = sigma(iroha[i], resha[j]);
      int tap = tapris.first;
      int ris = tapris.second;
      if (tap != 1) continue;
      if (tap == 1) {
        cod.insert(ris);
      }
    }
    if (cod.size() >= 2) {
      jinsei = -1;
    }
    if (cod.size() >= 1) {
      p.insert(*cod.begin());
    }
  }
  REP(j, 0, m) {
    set<int> cod;
    REP(i, 0, n) {
      PI tapris = sigma(iroha[i], resha[j]);
      int tap = tapris.first;
      int ris = tapris.second;
      if (tap != 1) continue;
      if (tap == 1) {
        cod.insert(ris);
      }
    }
    if (cod.size() >= 2) {
      jinsei = -1;
    }
    if (cod.size() >= 1) {
      q.insert(*cod.begin());
    }
  }
  if (DEBUG) {
    for(int pi: p) cerr << " " << pi;
    cerr << endl;
    for(int qi: q) cerr << " " << qi;
    cerr << endl;
  }
  if (jinsei == -1) {
    cout << "-1\n";
    return 0;
  }
  set<int> kirika;
  for (int v: p) kirika.insert(v);
  VI ans;
  for (int v: q) if (kirika.count(v)) ans.push_back(v);
  if (ans.size() != 1) {
    cout << "0\n";
    return 0;
  }
  cout << ans[0] << "\n";
}
