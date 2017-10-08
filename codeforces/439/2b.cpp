#include <algorithm>
#include <cassert>
#include <cctype>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll a, b;
  cin >> a >> b;
  if (b - a >= 10) {
    cout << "0\n";
    return 0;
  }
  ll cur = 1;
  for (ll i = a + 1; i <= b; ++i) {
    cur *= i % 10;
    cur %= 10;
  }
  cout << cur << "\n";
}
