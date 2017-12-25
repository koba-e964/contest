#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;


int gen = 0;

void rec(VI &ans, int l, int r, int k) {
  assert (k % 2 == 1);
  assert (k >= 1);
  assert (k <= 2 * (r - l) - 1);
  if (k == 1) {
    REP(i, l, r) {
      gen++;
      ans[i] = gen;
    }
    return;
  }
  int mid = (l + r) / 2;
  int x = 2 * min(mid - l - 1, (k - 3) / 2) + 1;
  int y = k - 1 - x;
  rec(ans, mid, r, y);
  rec(ans, l, mid, x);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  if (k > 2 * n - 1 || (k + 1) % 2 != 0) {
    cout << -1 << "\n";
    return 0;
  }
  VI ans(n);
  rec(ans, 0, n, k);
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n": " ");
  }
}
