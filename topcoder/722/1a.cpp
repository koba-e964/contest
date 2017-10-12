#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

class TCPhoneHome {
  int digits;
  vector<string> res;
  vector<ll> pow10;
  ll rec(int v, string pre) {
    bool poss = false;
    REP(i, 0, res.size()) {
      if (res[i] == pre) {
	return 0;
      }
      if (res[i].substr(0, pre.length()) == pre) {
	poss = true;
      }
    }
    if (not poss) {
      return pow10[digits - v];
    }
    if (v >= digits) {
      return 1;
    }
    ll tot = 0;
    for (char c = '0'; c <= '9'; ++c) {
      tot += rec(v + 1, pre + c);
    }
    return tot;
  }
public:
  long long validNumbers(int digits, vector <string> specialPrefixes) {
    this->digits = digits;
    this->res = specialPrefixes;
    this->pow10 = vector<ll>(digits + 1);
    this->pow10[0] = 1;
    REP(i, 1, digits + 1) {
      this->pow10[i] = 10 * this->pow10[i - 1];
    }
    return rec(0, "");
  }
};

int main(void) {
  int digits;
  cin >> digits;
  int n;
  cin >> n;
  vector<string> res(n);
  REP(i, 0, n) {
    cin >> res[i];
  }
  cout << TCPhoneHome().validNumbers(digits, res) << endl;
}
