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

const int N = 20010;
int n, m;
VI edges[N];
int col[N];
int dp[N][16];

// Colorful path
// https://docs.com/_-camypaper/03df6e8a-8a6b-48d5-9245-3143a639e2ed/yukicoder408
bool check() {
  REP(i, 0, n) {
    fill_n(dp[i], 16, 0);
  }
  REP(i, 0, edges[0].size()) {
    int v = edges[0][i];
    dp[v][1 << col[v]] = 1;
  }
  REP(bits, 1, 16) {
    REP(i, 0, n) {
      if (bits & (1 << col[i])) {
	int p_bits = bits ^ (1 << col[i]);
	REP(j, 0, edges[i].size()) {
	  int p = edges[i][j];
	  if (p == 0) {
	    continue;
	  }
	  dp[i][bits] |= dp[p][p_bits];
	}
      }
    }
  }
  bool ok = 0;
  REP(i, 0, edges[0].size()) {
    ok |= dp[edges[0][i]][15];
  }
  return ok;
}

int main(void){
  cin >> n >> m;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  REP(loop_cnt, 0, 512) {
    REP(i, 0, n) {
      col[i] = rand() % 4; // color by 4 colors
    }
    bool res = check();
    if (res) {
      cout << "YES" << endl;
      return 0;
    }
  }
  cout << "NO" << endl;
}
