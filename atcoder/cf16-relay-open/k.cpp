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

const int N = 100100;

VI edges[N];

// Reference: http://cf16-relay-open.contest.atcoder.jp/submissions/1002080

int c[N];

int ma = 0;

int dfs(int v, int p) {
  VI t(2, 0);
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == p) {
      continue;
    }
    int r = dfs(w, v);
    t.push_back(r);
  }
  sort(t.rbegin(), t.rend());
  ma = max(ma, t[0] + t[1] + c[v]);
  c[v] += t[0];
  return c[v];
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int p, q;
    cin >> p >> q;
    p--, q--;
    edges[p].push_back(q);
    edges[q].push_back(p);
  }
  // Find one of the longest path and counts vertices of index two.
  REP(i, 0, n) {
    c[i] = edges[i].size() == 2 ? 1 : 0;
  }
  dfs(0, -1);
  int tot = ma;
  REP(i, 0, n) {
    if (edges[i].size() == 1) { tot++; }
  }
  cout << tot << endl;
}
