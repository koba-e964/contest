#include <algorithm>
#include <iostream>

using namespace std;
typedef long long int ll;

int main(void){
  ll a, b, t;
  cin >> a >> b >> t;
  ll mi = (t + a - 1) / a * a;
  for (ll x = 0; x < a * b && x < t + b; x += b) {
    ll rem = t - x;
    ll tmp = x;
    if (rem > 0) {
      tmp += (rem + a - 1) / a * a;
    }
    mi = min(mi, tmp);
  }
  cout << mi << endl;
}
