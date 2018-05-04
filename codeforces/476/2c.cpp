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
  ll n, k, m, d;
  cin >> n >> k >> m >> d;
  ll ma = 0;
  REP(e, 1, d + 1) {
    // Arkady receives e times
    double dnum = (double)k * (double)(e - 1) + 1.0;
    if (n <= dnum) {
      continue;
    }
    ll num = k * (e - 1) + 1;
    ll hi = min(m, n / num);
    ll lo = n / (k * e + 1) + 1;
    if (lo <= hi)
      ma = max(ma, hi * e);
  }
  cout << ma << endl;
}
