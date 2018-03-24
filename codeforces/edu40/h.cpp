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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


const int N = 5001;
int c[N][2 * N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  n--;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  c[n][0] = 1;
  for (int i = n - 1; i >= 0; --i) {
    c[i][0] = 1;
    ll fac = a[i] * a[i] % mod;
    REP(j, 0, 2 * N - 2) {
      c[i][j + 2] = (c[i][j + 2] + fac * c[i + 1][j]) % mod;
    }
    ll tmp = 2 * a[i] % mod;
    REP(j, 0, n - i) {
      c[i][j + 1] = (c[i][j + 1] + tmp) % mod;
      if (j == n - i - 1) break;
      tmp = tmp * a[i + j + 1] % mod;
    }
  }
  if (0) {
    REP(i, 0, n + 1) {
      cerr << "C_"<<i<<":";
      REP(j, 0, 2 * n - 2 * i + 1) {
	cerr << " " << c[i][j];
      }
      cerr << endl;
    }
  }
  vector<ll> ans(2 * n + 1);
  REP(i, 0, 2 * n + 1) {
    ans[i] = c[0][i];
  }
  ll tmp = 1;
  REP(i, 1, n + 1) {
    tmp = tmp * a[i - 1] % mod;
    REP(j, 0, 2 * n + 1) {
      ans[j] = (ans[j] + tmp * c[i][j]) % mod;
    }
    REP(j, 0, 2 * n - 1) {
      ans[j + 2] = (ans[j + 2] + (mod - tmp) * c[i][j]) % mod;
    }
  }
  REP(i, 1, 2 * n + 1) {
    cout << ans[i] * ((mod + 1) / 2) % mod << (i == 2 * n ? "\n" : " ");
  }
}
