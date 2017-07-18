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

int n, m;
const int N = 1234;
VI children[N];
int p[N], c[N];


PI dfs(int v) {
  PI res;
  int mi = 1e8;
  int chc = 0;
  int tot = 0;
  if (children[v].size() == 0) {
    return PI(c[v], 0);
  }
  REP(i, 0, children[v].size()) {
    int w = children[v][i];
    PI sub = dfs(w);
    chc += 1;
    mi = min(mi, sub.first);
    tot += sub.first;
    res.second += sub.second;
  }
  if (v == 0) {
    mi = 0;
  }
  res.second += tot - chc * mi;
  res.first = mi;
  return res;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m;
  REP(i, 0, n - 1) {
    cin >> p[i];
    children[p[i]].push_back(i + 1);
  }
  REP(i, 0, m) {
    int bb, cc;
    cin >> bb >> cc;
    c[bb] = cc;
  }
  PI ans = dfs(0);
  cout << ans.first + ans.second << "\n";
}
