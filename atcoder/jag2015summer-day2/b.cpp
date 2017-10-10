#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;



int main(void) {
  int n;
  ll k;
  cin >> n >> k;
  ll pos = 0;
  REP(i, 0, n - 1) {
    int r = pos % (k - 1);
    ll q = pos / (k - 1);
    pos = q * k + r + 1;
  }
  cout << pos << endl;
}
