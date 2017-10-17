#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VI a(n);
  int ma = -1e9;
  int mi = 1e9;
  REP(i, 0, n) {
    cin >> a[i];
    ma = max(ma, a[i]);
    mi = min(mi, a[i]);
  }
  if (k == 1) {
    cout << mi << "\n";
    return 0;
  }
  if (k == 2) {
    cout << max(a[0], a[n - 1]) << "\n";
    return 0;
  }
  cout << ma << "\n";
}
