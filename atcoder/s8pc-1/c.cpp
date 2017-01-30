#include <algorithm>
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

int n, k;
const int N = 60;
ll d[N];
int x[N], y[N];
VI edges[N];

void dfs(int v, ll bits, ll cur, ll &ma) {
  ma = max(ma, cur);
  REP(j, 0, edges[v].size()) {
    int w = edges[v][j];
    if (bits & (1LL << w)) { continue; }
    dfs(w, bits | 1LL << w, cur + d[w], ma);
  }
}

int main(void){
  ll ma = 0;
  cin >> n >> k;
  REP(i, 0, n) { cin >> d[i]; }
  REP(i, 0, k) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
    edges[x[i]].push_back(y[i]);
    edges[y[i]].push_back(x[i]);
  }
  REP(i, 0, n) {
    dfs(i, 1LL << i, d[i], ma);
  }
  cout << ma << endl;
}
