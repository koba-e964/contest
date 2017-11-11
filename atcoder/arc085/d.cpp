#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 2437;
ll dp[N][N];
ll ma_fold[N][N]; // row
ll mi_fold[N][N]; // column


const ll inf = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll z, w;
  cin >> n >> z >> w;
  VL a(n+2);
  a[0] = z;
  a[1] = w;
  REP(i, 0, n) {
    cin >> a[i+2];
  }
  REP(i, 0, N){
    REP(j, 0, N) {
      mi_fold[i][j] = inf;
    }
  }
  for (int i = n + 1; i >= 0; --i) {
    for (int j = n + 1; j >= 0; --j) {
      if (i == j) { continue; }
      if (i == n + 1 || j == n + 1) {
	dp[i][j] = abs(a[i] - a[j]);
      } else if (i < j) {
	ll ma = ma_fold[j+1][j];
	dp[i][j]= ma;
      } else {
	dp[i][j] = mi_fold[i][i + 1];
      }
      ma_fold[i][j] = max(ma_fold[i+1][j], dp[i][j]);
      mi_fold[i][j] = min(mi_fold[i][j+1], dp[i][j]);
    }
  }
  if (0) {
    cerr << "dp:";
    REP(i, 0, n + 2) {
      REP(j, 0, n + 2) {
	cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
    cerr << "mi_fold:";
    REP(i, 0, n + 2) {
      REP(j, 0, n + 2) {
	cerr << " " << mi_fold[i][j];
      }
      cerr << endl;
    }
  }
  cout << dp[0][1] << endl;
}
