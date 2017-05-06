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

int col[N];
int wg[N];

int dfs(int v, int par = -1) {
  int sum = 0;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    int sub = dfs(w, v);
    sum += sub;
  }
  wg[v] = 1 + sum;
  return 1 + sum;
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  dfs(0);
  int win = 0;
  REP(i, 0, n) {
    int odd = 0;
    int sum = 0;
    REP(j, 0, edges[i].size()) {
      int w = edges[i][j];
      if (wg[w] > wg[i]) { continue; }
      if (wg[w] % 2 == 1) {
	sum += 1;
	odd += 1;
      }
    }
    if (i != 0) {
      odd += (n + 1 + sum) % 2;
    }
    if (odd >= 2) {
      cout << "First" << endl;
      return 0;
    }
  }
  cout << "Second" << endl;
}
