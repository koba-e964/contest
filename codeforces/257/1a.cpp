#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

ll f(ll n, ll m, ll x, ll y) {
  x = max(x, 1LL);
  y = max(y, 1LL);
  return (n / x) * (m / y);
}

int main(void) {
  ll n, m, k;
  cin >> n >> m >> k;
  k += 2;
  if (k > n + m) {
    cout << -1 << endl;
    return 0;
  }
  ll ma = 0;
  for (ll i = 1; i * i <= n; ++i) {
    ma = max(ma, f(n, m, i, k - i));
    ma = max(ma, f(n, m, n / i, k - n / i));
  }
  cout << ma << endl;
}
