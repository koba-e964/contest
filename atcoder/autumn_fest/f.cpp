#include <cmath>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;


int main(void) {
  int n;
  cin >> n;
  vector<double> a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  bool neg = false;
  REP(i, 0, n) {
    neg ^= a[i] < 0;
  }
  VI sgn(n);
  sgn[0] = 1;
  sgn[1] = -1;
  REP(i, 2, n) {
    if (neg) {
      sgn[i] = abs(a[i]) > 1 ? -1 : 1;
    } else {
      sgn[i] = abs(a[i]) > 1 ? 1 : -1;
    }
  }
  VI ans(n - 1);
  int mi = 1;
  int ma = n - 1;
  REP(i, 0, n - 1) {
    int cur = -sgn[i + 1];
    if (i == n - 2) {
      ans[i] = ma;
    } else if (sgn[i + 2] != cur) {
      ans[i] = mi;
      mi += 1;
    } else {
      ans[i] = ma;
      ma -= 1;
    }
  }
  REP(i, 0, n - 1) {
    cout << ans[i] << endl;
  }
}
