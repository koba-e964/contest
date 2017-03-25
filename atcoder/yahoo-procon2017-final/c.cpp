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

// sqrt decomposition

const int W = 320;
const int N = 102400;


int dp[W][N];
int a[N];
int bias[W];
int n, m, q;

void update(int idx, int d) {
  int olda = a[idx];
  int newa = (olda + d) % m;
  int b = idx / W;
  dp[b][olda] -= 1;
  dp[b][newa] += 1;
  a[idx] = newa;
}


int main(void){
  cin >> n >> m >> q;
  REP(i, 0, n) {
    cin >> a[i];
    a[i] %= m;
  }
  REP(i, 0, n) {
    int b = i / W;
    dp[b][a[i]] += 1;
  }
  REP(loop_cnt, 0, q) {
    int l, r, d;
    cin >> l >> r >> d;
    l--;
    d %= m;
    int lb = (l + W - 1) / W;
    int rb = r / W;
    int cnt = 0;
    if (lb <= rb) {
      REP(i, lb, rb) {
	bias[i] += m - d;
	bias[i] %= m;
      }
      REP(i, l, lb * W) {
	update(i, d);
      }
      REP(i, rb * W, r) {
	update(i, d);
      }
      // count
      REP(i, lb, rb) {
	cnt += dp[i][bias[i]];
      }
      REP(i, l, lb * W) {
	if (a[i] == bias[i / W]) {
	  cnt += 1;
	}
      }
      REP(i, rb * W, r) {
	if (a[i] == bias[i / W]) {
	  cnt += 1;
	}
      }
    } else {
      REP(i, l, r) {
	update(i, d);
      }
      REP(i, l, r) {
	if (a[i] == bias[i / W]) {
	  cnt += 1;
	}
      }
    }
    cout << cnt << endl;
  }
}
