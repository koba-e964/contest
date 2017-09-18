#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const int N = 123456;

VI edges[N];
int depth[N];

void dfs(int v) {
  int mi = 1e8;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    dfs(w);
    mi = min(mi, depth[w]);
  }
  if (mi == (int) 1e8) {
    depth[v] = 0;
  } else {
    depth[v] = mi + 1;
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
    if (a[i] >= 0) {
      edges[a[i]].push_back(i);
    }
  }
  fill_n(depth, N, -1);
  REP(i, 0, n) {
    if (depth[i] == -1) {
      dfs(i);
    }
  }
  int tot = 0;
  REP(i, 0, n) {
    tot += depth[i] < k || a[i] == -1 ? 1 : 0;
  }
  cout << tot << "\n";
}
