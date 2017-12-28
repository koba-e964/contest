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

const int N = 100100;
VI edges[N];
VI rev[N];
int outdeg[N];
int e[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> e[i];
  }
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    edges[u].push_back(v);
    rev[v].push_back(u);
    outdeg[u] += 1;
  }
  queue<int> zero[2];
  REP(i, 0, n) {
    if (outdeg[i] == 0) {
      zero[e[i]].push(i);
    }
  }
  int ans = 0;
  int mode = 0;
  int rest = n;
  while (rest > 0) {
    bool rm = false;
    while (not zero[mode].empty()) {
      rm = true;
      int v = zero[mode].front(); zero[mode].pop();
      REP(i, 0, rev[v].size()) {
	int u = rev[v][i];
	outdeg[u] -= 1;
	if (outdeg[u] == 0) {
	  zero[e[u]].push(u);
	}
      }
      rest -= 1;
    }
    if (mode == 1 && rm) {
      ans += 1;
    }
    mode = 1 - mode;
  }
  cout << ans << "\n";
}
