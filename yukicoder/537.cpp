#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

string calc(ll a, ll b) {
  stringstream ss;
  ss << a << b;
  return ss.str();
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  set<string> cand;
  REP(i, 1, 1.1e6) {
    if (n % i == 0) {
      cand.insert(calc(i, n / i));
      cand.insert(calc(n / i, i));
    }
  }
  cout << cand.size() << "\n";
}
