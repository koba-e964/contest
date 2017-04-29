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

ll w0;

void dfs(const vector<vector<ll> > &diff, int v, ll w_rem, ll tot, ll &ma) {
  ll round_sum = 0;
  if (v == 4) {
    ma = max(ma, tot);
    return;
  }
  REP(i, 0, diff[v].size() + 1) {
    if (w_rem < (w0 + v) * i) {
      break;
    }
    dfs(diff, v + 1, w_rem - (w0 + v) * i, tot + round_sum, ma);
    if (i < (int) diff[v].size()) {
      round_sum += diff[v][i];
    }
  }
}

int main(void){
  int n;
  ll w_tot;
  cin >> n >> w_tot;
  vector<ll> w(n), v(n);
  REP(i, 0, n) {
    cin >> w[i] >> v[i];
  }
  vector<vector<ll> > diff(4);
  w0 = w[0];
  REP(i, 0, n) {
    diff[w[i] - w[0]].push_back(v[i]);
  }
  REP(i, 0, 4) {
    sort(diff[i].rbegin(), diff[i].rend());
  }
  ll ma = 0;
  dfs(diff, 0, w_tot, 0, ma);
  cout << ma << endl;
}
