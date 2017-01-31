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

const int N = 40;
VI edges[N];

void fail(void) {
  cout << "NO" << endl;
  exit(0);
}

ll px[N], py[N];

void dfs(int v, int p, ll d, int sx, int sy, ll x, ll y) {
  px[v] = x;
  py[v] = y;
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  int cur = 0;
  for (auto w: edges[v]) {
    if (w == p) {
      continue;
    }
    if (dx[cur] == sx && dy[cur] == sy) {
      cur++;
    }
    ll nx = x + dx[cur] * d / 2;
    ll ny = y + dy[cur] * d / 2;
    dfs(w, v, d / 2, -dx[cur], -dy[cur], nx, ny);
    cur++;
  }
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  // any vertex whose index >= 5 ?
  REP(i, 0, n) {
    if (edges[i].size() >= 5) {
      fail();
    }
  }
  dfs(0, -1, 1LL << (n + 1), 0, 0, 0LL, 0LL);
  cout << "YES" << endl;
  REP(i, 0, n) {
    cout << px[i] << " " << py[i] << endl;
  }
}
