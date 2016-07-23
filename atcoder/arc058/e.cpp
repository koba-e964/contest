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


const ll mod = 1e9 + 7;

const int N = 41;
const int S = 1 << 18;
ll dp[N][S];
int next_tbl[S][11];
int n, x, y, z;
int qs;

int next_state_nfa(int q, int s) {
  int lim;
  assert (q < qs);
  if (q < x) {
    lim = x;
  } else if (q < x + y) {
    lim = x + y;
  } else if (q < x + y + z) {
    lim = x + y + z;
  } else {
    return x + y + z;
  }
  
  if (q + s <= lim) {
    return q + s;
  }
  return -1;
}

int next_state(int q, int s) {
  int bits = 1;
  REP(i, 0, qs) {
    if (q & (1 << i)) {
      int r = next_state_nfa(i, s);
      if (r >= 0) bits |= 1 << r;
    }
  }
  return bits;
}

int main(void){
  cin >> n >> x >> y >> z;
  qs = x + y + z + 1;
  dp[0][1] = 1;
  REP(bits, 0, 1 << qs) {
    REP(s, 1, 11) {
      next_tbl[bits][s] = next_state(bits, s);
    }
  }
  REP(i, 1, n + 1) {
    REP(bits, 0, 1 << qs) {
      REP(s, 1, 11) {
	int next = next_tbl[bits][s];
	dp[i][next] += dp[i - 1][bits];
	dp[i][next] %= mod;
      }
    }
  }
  ll sum = 0;
  REP(bits, 1 << (qs - 1), 1 << qs) { // final states
    sum += dp[n][bits];
    sum %= mod;
  }
  cout << sum << endl;
}
