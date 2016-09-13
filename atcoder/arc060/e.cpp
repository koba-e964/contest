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

const int N = 100010;
const int B = 20;
const int DEBUG = 0;
int dp[N][B];


int check(int a, int day) {
  REP(i, 0, B) {
    if (day & (1 << i)) {
      a = dp[a][i];
    }
  }
  return a;
}

int main(void){
  int n, l, q;
  cin >> n;
  VI x(n);
  REP(i, 0, n) {
    cin >> x[i];
  }
  cin >> l >> q;
  int cur = 0;
  // Shakutori method
  REP(i, 0, n) {
    while (cur < n && x[i] + l >= x[cur]) {
      cur++;
    }
    dp[i][0] = cur - 1;
    assert (i == n - 1 || cur - i >= 2);
    assert (cur <= n);
  }
  REP(b, 1, B) {
    REP(i, 0, n) {
      dp[i][b] = dp[dp[i][b - 1]][b - 1];
    }
  }
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "dp[" << i << "]:";
      REP(b, 0, B) {
	cerr << " " << dp[i][b];
      }
      cerr << endl;
    }
  }
  REP(i, 0, q) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    if (a > b) {
      swap(a, b);
    }
    int lo = 0, hi = 1 << (B - 1);
    while (hi - lo > 1) {
      int mid = (lo + hi) / 2;
      if (check(a, mid) >= b) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    cout << hi << endl;
  }
}
