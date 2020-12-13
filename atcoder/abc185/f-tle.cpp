#pragma GCC optimize ("-O3", "unroll-loops")

#include <cassert>
#include <iomanip>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

const int N = 300001;
int a[N];

__attribute__((target("avx512vbmi")))
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  REP(i, 0, n) cin >> a[i];
  for (int i = n - 1; i >= 0; i--) a[i] ^= a[i + 1];
  REP(i, 0, q) {
    int t, x, y;
    cin >> t >> x >> y;
    x--;
    if (t == 1) {
      if (2 * x < n) {
        REP(j, 0, x + 1) a[j] ^= y;
      } else {
        REP(j, x + 1, n + 1) a[j] ^= y;
      }
    } else {
      int s = a[y] ^ a[x];
      cout << s << "\n";
    }
  }
}
