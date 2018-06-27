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

int main(void) {
  ll x;
  cin >> x;
  ll ans = 0;
  int coin[] = {100, 20, 10, 5, 1};
  for (int c: coin) {
    ans += x / c;
    x %= c;
  }
  cout << ans << "\n";
}
