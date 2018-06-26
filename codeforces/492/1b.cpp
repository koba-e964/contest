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
  int n;
  cin >> n;
  VI a(2 * n);
  REP(i, 0, 2 * n) {
    cin >> a[i];
  }
  int tot = 0;
  REP(i, 0, n) {
    int idx = -1;
    REP(j, 2 * i + 1, 2 * n) {
      if (a[2 * i] == a[j]) {
	idx = j;
	break;
      }
    }
    for (int j = idx - 1; j >= 2 * i + 1; --j) {
      tot++;
      swap(a[j + 1], a[j]);
    }
  }
  cout << tot << "\n";
}
