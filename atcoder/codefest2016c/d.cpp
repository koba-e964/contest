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

const int H = 310;
string c[H];
const int inf = 1e9;
int dp[H][H][H];
int diff[H][H][H];

int h, w;

void build_tbl(int v) {
  REP(i, 0, h + 1) {
    REP(j, 0, h + 1) {
      int tmp = 0;
      if (i >= 1 && j >= 1) {
	tmp = diff[v][i - 1][j - 1] + (c[i - 1][v] == c[j - 1][v + 1] ? 1 : 0);
      }
      diff[v][i][j] = tmp;
    }
  }
}

int calc_tbl(int v) {
  dp[v][0][0] = 0;
  REP(i, 0, h + 1){
    REP(j, 0, h + 1) {
      if(i==0 && j==0) continue;
      int mi = inf;
      if (i >= 1)
	mi = min(mi, dp[v][i - 1][j] + diff[v][i][j]);
	if (j >= 1)
	  mi = min(mi, dp[v][i][j - 1] + diff[v][i][j]);
	dp[v][i][j] = mi;
      }
    }
  return dp[v][h][h];
}
int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> c[i];
  }
  REP(i, 0, w - 1) {
    build_tbl(i);
  }
  REP(i, 0, h + 1){
    REP(j, 0, h + 1) {
      REP(k, 0, h + 1) {
	dp[i][j][k] = inf;
      }
    }
  }
  int sum = 0;
  REP(i, 0, w - 1) {
    sum += calc_tbl(i);
  }
  cout << sum << endl;
}
