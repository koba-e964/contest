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
typedef pair<int, ll> PIL;
const ll mod = 1e9 + 7;

const int N = 100100;
vector<PIL> edges[N];

bool vis[N];
bool contra;
ll pot[N];

void dfs(int v, ll p) {
  if (vis[v]) {
    if (pot[v] != p) {
      contra = true;
    }
    return;
  }
  vis[v] = true;
  pot[v] = p;
  REP(i, 0, edges[v].size()) {
    PIL wc = edges[v][i];
    int w = wc.first;
    ll c = wc.second;
    dfs(w, p + c);
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int l, r, d;
    cin >> l >> r >> d;
    edges[l].push_back(PIL(r, d));
    edges[r].push_back(PIL(l, -d));
  }
  REP(i, 0, n) {
    if (not vis[i]) {
      dfs(i, 0);
    }
  }
  cout << (contra ? "No" : "Yes") << endl;
}
