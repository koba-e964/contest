#include <iostream>

using namespace std;
typedef long long int ll;

ll query(ll lo, ll hi, int &nq) {
  while (hi - lo >= 2) {
    ll mid = (hi + lo) / 2;
    cout << "? " << mid - nq << endl;
    int res;
    cin >> res;
    nq++;
    if (res == 0) {
      return mid;
    }
    if (res > 0) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  return lo;
}

int main(void){
  cin.sync_with_stdio(false);
  int nq = 0;
  ll res = query(10, 200, nq);
  if (res <= 98) {
    cout << "! " << res << endl;
    return 0;
  }
  ll val = query(98, 1e9 + 1, nq);
  cout << "! " << val << endl;
}
