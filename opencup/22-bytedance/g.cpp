#include <cassert>
#include <iomanip>
#include <iostream>


using namespace std;
typedef long long int ll;

const int N = 3.1e6;
bool pr[N];
ll psum[N];

void init() {
  for (int i = 2; i < N; i++) pr[i] = 1;
  for (int i = 2; i < N; i++) {
    if (!pr[i]) continue;
    for (int j = 2 * i; j < N; j += i) pr[j] = 0;
  }
  for (int i = 1; i < N; i++) psum[i] = psum[i - 1] + pr[i];
}

ll dfs(ll divsum, ll pmin, ll n) {
  ll ans = 1;
  ll p = pmin;
  for (; p <= min(1 + divsum, n); p++) {
    if (p * p >= n + 1) break;
    if (!pr[p]) continue;
    int e = 1;
    ll v = n / p;
    ll me = 1 + p;
    while (v >= 1) {
      ans += dfs(divsum * me, p + 1, v);
      me = me * p + 1;
      v /= p;
      e++;
    }
  }
  if (p <= min(1 + divsum, n) && p * p >= n + 1) {
    ans += psum[min(1 + divsum, n)] - psum[p - 1];
  }
  return ans;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  ll n;
  cin >> n;
  cout << dfs(1, 2, n) << endl;
}
