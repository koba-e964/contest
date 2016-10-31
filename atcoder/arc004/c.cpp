#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

ll gcd(ll x, ll y) {
  if (y == 0) { return x; }
  return gcd(y, x % y);
}

PL add(const PL &a, const PL &b) {
  ll numer = a.first * b.second + a.second * b.first;
  ll denom = a.second * b.second;
  ll g = gcd(numer, denom);
  numer /= g, denom /= g;
  return PL(numer, denom);
}

int main(void){
  ll x,y;
  cin >> x;
  cin.ignore();
  cin >> y;
  ll g = gcd(x, y);
  x /= g, y /= g;
  PL lb = add(PL(2 * x, y), PL(-1, 1));
  ll q = lb.first / lb.second;
  vector<PL> res;
  REP(i, 0, 3) {
    ll n = q + i;
    PL m = add(PL(n + 1, 2), PL(-x, y));
    m.first *= n;
    m = add(m, PL(0, 1));
    ll ml = m.first;
    if (m.second == 1 && 1 <= ml && ml <= n) {
      res.push_back(PL(n, ml));
    }
  }
  if (res.size()) {
    REP(i, 0, res.size()) {
      cout << res[i].first << " " << res[i].second << endl;
    }
  } else {
    cout << "Impossible" << endl;
  }
}
