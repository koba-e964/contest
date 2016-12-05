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

const int N = 200100;
int a[N], d[N];
const int H = 3002;

int n, k;
int board[H][H];
int acc[H][H];

bool check(int x) {
  REP(i, 0, H - x) {
    REP(j, 0, H - x) {
      int cnt = acc[i + x][j + x] - acc[i + x][j] - acc[i][j + x] + acc[i][j];
      if (cnt >= k) {
	return true;
      }
    }
  }
  return false;
}

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll fact[N];
ll inv_fact[N];

ll comb(int x, int y) {
  if (y < 0 || y > x) {
    return 0;
  }
  return fact[x] * inv_fact[x - y] % mod * inv_fact[y] % mod;
}

int main(void){
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> a[i] >> d[i];
    board[a[i]][d[i]]++;
  }
  REP(i, 1, H) {
    REP(j, 1, H) {
      acc[i][j] = acc[i][j - 1] + acc[i - 1][j] - acc[i - 1][j - 1]
	+ board[i - 1][j - 1];
    }
  }
  fact[0] = inv_fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
    inv_fact[i] = invmod(fact[i]);
  }
  int lo = -1;
  int hi = H;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    if (check(mid)) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << hi - 1 << endl;
  ll tot = 0;
  REP(i, 0, H - hi) {
    REP(j, 0, H - hi) {
      int cnt = acc[i + hi][j + hi] - acc[i + hi][j] - acc[i][j + hi]
	+ acc[i][j];
      int cnt1 = acc[i + hi][j + hi - 1] - acc[i + hi][j] - acc[i][j + hi - 1]
	+ acc[i][j];
      int cnt2 = acc[i + hi - 1][j + hi] - acc[i + hi - 1][j] - acc[i][j + hi]
	+ acc[i][j];
      int cnt3 = acc[i + hi - 1][j + hi - 1] - acc[i + hi - 1][j]
	- acc[i][j + hi - 1] + acc[i][j];
      tot = (tot + comb(cnt, k)
	     - (j >= 1 ? comb(cnt1, k) : 0)
	     - (i >= 1 ? comb(cnt2, k) : 0)
	     + (i >= 1 && j >= 1 ? comb(cnt3, k) : 0) + 2 * mod) % mod;
    }
  }
  cout << tot << endl;
}
