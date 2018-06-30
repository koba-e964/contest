#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;


int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);
  int n;
  cin >> n;
  vector<ll> a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i] -= i + 1;
  }
  int maxl = 0;
  VI m(n + 1, -1);
  REP(i, 0, n) {
    if (a[i] < 0) { continue; }
    int lo = 0;
    int hi = maxl + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (a[m[mid]] <= a[i]) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    int l = lo + 1;
    maxl = max(maxl, l);
    m[l] = i;
  }
  cout << n - maxl << endl;
}
