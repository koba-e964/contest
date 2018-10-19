#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n); REP(i, 0, n) cin >> a[i];
  int odd = 0;
  REP(i, 0, n) odd += a[i] % 2;
  odd %= 2;
  if (odd == n) {
    cout << -1 << endl;
  } else {
    cout << odd << endl;
  }
}
