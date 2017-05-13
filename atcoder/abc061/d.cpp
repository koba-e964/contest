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

const int N = 1000;
ll dist[N];
ll inf = 1e16;
vector<pair<int, ll> > edges[N];
vector<int> rev[N];

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    dist[i] = inf;
  }
  dist[0] = 0;
  REP(i, 0, m) {
    ll a, b, c;
    cin >> a >> b >> c;
    a--, b--;
    edges[a].push_back(make_pair(b, -c));
    rev[b].push_back(a);
  }
  vector<bool> tone(n);
  queue<int> que;
  que.push(n - 1);
  while (not que.empty()) {
    int t = que.front(); que.pop();
    if (tone[t]) { continue; }
    tone[t] = true;
    REP(i, 0, rev[t].size()) {
      int u = rev[t][i];
      que.push(u);
    }
  }
  bool neg_cycle = false;
  REP(iter, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, edges[i].size()) {
	pair<int, ll> e = edges[i][j];
	int w = e.first;
	if (dist[i] == inf) { continue; }
        ll tmp = dist[i] + e.second;
	if (dist[w] > tmp) {
	  dist[w] = tmp;
	  if (iter == n - 1 && tone[w]) {
	    neg_cycle = true;
	  }
	}
      }
    }
  }
  if (neg_cycle) {
    cout << "inf" << endl;
  } else {
    cout << -dist[n - 1] << endl;
  }
}
