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

const int D = 11;
const int N = (1 << (D + 1)) - 1;

void dfs(int x, int y, int d, int l, int q, vector<PI> &res) {
  int dx = q == 0 ? l : 0;
  int dy = q == 0 ? 0 : l;
  int f = q == 0 ? 2 : 1;
  if (d < D) {
    dfs(x - dx, y - dy, d + 1, l / f, 1 - q, res);
  }
  res.push_back(PI(x + (q == 0 ? 1 : 0), y + (q == 0 ? 0 : 1)));
  if (d < D) {
    dfs(x + dx, y + dy, d + 1, l / f, 1 - q, res);
  }
}


int main(void){
  vector<PI> res;
  dfs(128, 128, 0, 64, 0, res);
  assert (res.size() == N);
  int mi = 10000;
  REP(i, 0, N) {
    mi = min(mi, res[i].first);
  }
  REP(i, 0, N) {
    res[i].first -= mi;
  }
  mi = 10000;
  REP(i, 0, N) {
    mi = min(mi, res[i].second);
  }
  REP(i, 0, N) {
    res[i].second -= mi;
  }
  REP(i, 0, N) {
    cout << res[i].first << " " << res[i].second << endl;
  }
}
