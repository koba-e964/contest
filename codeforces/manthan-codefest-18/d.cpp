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
const ll mod = 1e9 + 7;

const int N = 200100;
VI g[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(_, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  queue<set<int> > que;
  {
    set<int> q;
    q.insert(0);
    que.push(q);
  }
  int cur = 0;
  set<int> remnant;
  vector<bool> vis(n);
  while (cur < n && (not remnant.empty() || not que.empty())) {
    if (remnant.empty()) {
      remnant = que.front(); que.pop();
    }
    int v = a[cur];
    if (remnant.count(v) == 0) {
      cout << "No" << endl;
      return 0;
    }
    remnant.erase(v);
    vis[v] = true;
    set<int> adj;
    for (int w: g[v]) {
      if (not vis[w]) adj.insert(w);
    }
    if (adj.size() != 0) {
      que.push(adj);
    }
    cur++;
    if (0) {
      cerr << "remnant:";
      for (int k: remnant) cerr << " " << k;
      cerr << endl;
      queue<set<int> > qcp(que);
      while(!qcp.empty()){
	set<int> k=qcp.front();qcp.pop();
	cerr << "[";
	for (int w: k) cerr << " " << w;
	cerr << "] ";
      }
      cerr << endl;
    }
  }
  cout << "Yes" << endl;
}
