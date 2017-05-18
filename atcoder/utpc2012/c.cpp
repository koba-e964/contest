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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 500;
set<int> adj[N];

bool dfs(int v, vector<int> &vis, int p = -1) {
  if (vis[v]) {
    return false;
  }
  vis[v] = true;
  for (set<int>::iterator it = adj[v].begin(); it != adj[v].end(); ++it) {
    int w = *it;
    if (w == p) { continue; }
    bool res = dfs(w, vis, v);
    if (not res) { return false; }
  }
  return true;
}


int main(void){
  int n, m;
  cin >> n >> m;
  int e = n * (n - 1) / 2;
  if (n < N) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	if (i != j) {
	  adj[i].insert(j);
	}
      }
    }
  }
  REP(i, 0, m) {
    int s, t;
    cin >> s >> t;
    if (n >= N) {
      cout << "no" << endl;
      continue;
    }
    s--, t--;
    if (adj[s].count(t) == 0) {
      adj[s].insert(t);
      adj[t].insert(s);
      e += 1;
    } else {
      adj[s].erase(t);
      adj[t].erase(s);
      e -= 1;
    }
    if (e >= n) {
      cout << "no" << endl;
      continue;
    }
    // Check
    bool ok = true;
    VI vis(n);
    REP(j, 0, n) {
      if (vis[j]) { continue; }
      ok &= dfs(j, vis);
      if (not ok) {
	break;
      }
    }
    cout << (ok ? "yes" : "no") << endl;
  }
}
