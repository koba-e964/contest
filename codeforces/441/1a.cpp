#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

ll numerology(ll x) {
  ll tot = 0;
  while (x > 0) {
    tot += x % 10;
    x /= 10;
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  VL cand;
  REP(i, 0, 100) {
    ll x = n - i;
    if (x > 0) {
      if (x + numerology(x) == n) {
	cand.push_back(x);
      }
    }
  }
  sort(cand.begin(), cand.end());
  cout << cand.size() << "\n";
  REP(i, 0, cand.size()) {
    cout << cand[i] << "\n";
  }
}
