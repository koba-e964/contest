#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
#include <iomanip>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 100100;
VI g[N];
int sz[N];
int dep[N];

int dfs(int v, int par, int d = 0) {
  int s = 1;
  dep[v] = d;
  for (int w: g[v]) {
    if (par == w) continue;
    s += dfs(w, v, d + 1);
  }
  sz[v] = s;
  return s;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    g[x].push_back(y);
    g[y].push_back(x);
  }
  dfs(0, -1);
  double num = 0;
  double den = 0;
  REP(i, 1, n) {
    num += (ll) sz[i] * dep[i];
    den += dep[i];
  }
  cout << setprecision(15) << n - num / den << endl;
}
