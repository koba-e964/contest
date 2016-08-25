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


const int N = 1019;
const int Q = 100100;
int tbl[N][N];
int ans[Q];
VI edges[Q];
int n, m, q;
pair<int, PI> ops[Q];
void dfs(int v, int tot) {
  //cerr << "vtot: " << v << " " << tot << endl;
  int old_tot = tot;
  int old_st = 0;
  int i = ops[v].second.first;
  int j = ops[v].second.second;
  switch(ops[v].first) {
  case 1:
    old_st = tbl[i][j];
    tbl[i][j] = 1;
    tot += 1 - old_st;
    break;
  case 2:
    old_st = tbl[i][j];
    tbl[i][j] = 0;
    tot += - old_st;
    break;
  case 3:
    REP(l, 1, m + 1) {
      tbl[i][l] = 1 - tbl[i][l];
      tot += 2 * tbl[i][l] - 1;
    }
    break;
  default:
    {}
  }
  ans[v] = tot;
  REP(l, 0, edges[v].size()) {
    //cerr << "reccall: " << edges[v][l] << endl;
    dfs(edges[v][l], tot);
  }
  // invert
  switch(ops[v].first) {
  case 1:
    tbl[i][j] = old_st;
    tot -= 1 - old_st;
    break;
  case 2:
    tbl[i][j] = old_st;
    tot += old_st;
    break;
  case 3:
    REP(l, 1, m + 1) {
      tbl[i][l] = 1 - tbl[i][l];
      tot += 2 * tbl[i][l] - 1;
    }
    break;
  default:
    {}
  }
  assert (tot == old_tot);
}

int main(void){
  cin >> n >> m >> q;
  REP(i, 1, q + 1) {
    int p;
    cin >> p;
    int a = 0, b = 0;
    switch (p) {
    case 1:
    case 2:
      cin >> a >> b;
      ops[i] = (pair<int, PI>(p, PI(a, b)));
      break;
    case 3:
    case 4:
      cin >> a;
      ops[i] = (pair<int, PI>(p, PI(a, b)));
    }
  }
  REP(i, 0, q + 1) {
    if (i + 1 <= q && ops[i + 1].first != 4) {
      edges[i].push_back(i + 1);
    }
  }
  for (int i = q; i >= 1; --i) {
    if (ops[i].first == 4) {
      // k -> i
      edges[ops[i].second.first].push_back(i);
    }
  }
  dfs(0, 0);
  REP(i, 1, q + 1) {
    cout << ans[i] << endl;
  }
}
