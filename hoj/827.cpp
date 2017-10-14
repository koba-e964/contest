#include <iostream>

using namespace std;
typedef long long int ll;



int main(void) {
  ll n, d;
  cin >> n >> d;
  int x = 0;
  for (int i = 62; i >= 0; --i) {
    ll mask = 1LL << i;
    if (d == mask) {
      x ^= 1;
      break;
    }
    if (d > mask) {
      d = 2 * mask - d; // Note: overflowing
      x ^= 1;
    }
  }
  cout << (x ? "TANI" : "YAMA") << endl;
}
