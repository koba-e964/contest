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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 100000;
VI edges[N];
int par[N];
int dep[N];

void dfs(int v, int p, int d) {
  par[v] = p;
  dep[v] = d;
  REP(i, 0, edges[v].size()) {
    int u = edges[v][i];
    if (u != p) {
      dfs(u, v, d + 1);
    }
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
  dfs(0, -1, 0);
  int maxi = 0;
  REP(i, 1, n) {
    if (dep[maxi] < dep[i]) maxi = i;
  }

  int sp = maxi;
  dfs(sp, -1, 0);
  maxi = 0;
  REP(i, 1, n) {
    if (dep[maxi] < dep[i]) maxi = i;
  }
  cout << sp + 1 << " " << maxi + 1 << endl;
}
