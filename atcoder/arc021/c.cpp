#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

typedef pair<ll, int> PLI;

// Took a look at the editorial
int main(void){
  int k;
  int n;
  cin >> k >> n;
  VL a(n), d(n);
  REP(i, 0, n) {
    cin >> a[i] >> d[i];
  }
  ll lo = 1;
  ll hi = 2e11;
  while (hi - lo > 1) {
    ll mid = (lo + hi) / 2;
    ll cnt = 0;
    REP(i, 0, n) {
      cnt += a[i] < mid ? (mid - a[i] - 1 + d[i]) / d[i] : 0;
    }
    if (cnt <= k) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  ll sum = 0;
  ll totcnt = 0;
  int just = 0;
  REP(i, 0, n) {
    if (a[i] > lo) { continue; }
    ll cnt = (lo - a[i] - 1 + d[i]) / d[i];
    totcnt += cnt;
    sum += a[i] * cnt + (cnt - 1) * cnt / 2 * d[i];
    just += (lo - a[i]) % d[i] == 0 ? 1 : 0;
  }
  assert (totcnt <= k);
  assert(k <= totcnt + just);
  sum += (k - totcnt) * lo;
  cout << sum << endl;
}
