#include <iostream>
#include <utility>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef pair<ll, ll> PL;

ll gcd(ll x, ll y) {
  return y == 0 ? x : gcd(y, x % y);
}

PL add(PL x, PL y) {
  PL w = PL(x.first * y.second + x.second * y.first, x.second * y.second);
  ll g = gcd(w.first, w.second);
  w.first /= g;
  w.second /= g;
  return w;
}

PL inv(PL x) {
  return x.first > 0 ? PL(x.second, x.first) : PL(-x.second, -x.first);
}

PL fr_lcm(PL x, PL y) {
  ll numer = x.first / gcd(x.first, y.first) * y.first;
  ll denom = gcd(x.second, y.second);
  return PL(numer, denom);
}

int main(void){
  ll t1, t2, t3;
  cin >> t1 >> t2 >> t3;
  PL mi = PL(1e16, 1);
  REP(i, 0, 2) {
    REP(j, 0, 2) {
      PL b = PL(1, t1);
      PL d = PL(-1 + 2 * i, t2);
      PL f = PL(-1 + 2 * j, t3);
      PL b_d = inv(add(b, d));
      PL b_f = inv(add(b, f));
      PL res = fr_lcm(b_d, b_f);
      if ((double)mi.first / mi.second > (double)res.first / res.second) {
	mi = res;
      }
    }
  }
  cout << mi.first << "/" << mi.second << endl;
}
