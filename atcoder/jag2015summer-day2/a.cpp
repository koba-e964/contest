#include <algorithm>
#include <iostream>


using namespace std;
typedef long long int ll;

/*
 *  min{b | exists e. q divides b^e}
 */
ll calc(ll q) {
  ll b = 1;
  ll cur = 2;
  while (1) {
    if (q % cur == 0) {
      if (b % cur != 0) {
	b *= cur;
      }
      q /= cur;
      continue;
    }
    if (cur * cur > q) {
      b *= q;
      break;
    }
    ++cur;
  }
  return b;
}

int main(void){
  ll p, q;
  cin >> p >> q;
  ll g = __gcd(p, q);
  p /= g;
  q /= g;
  cout << calc(q) << endl;
}
