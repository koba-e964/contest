#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

ll mod = 129402307;

ll readMod(const string &s, ll m) {
  ll sum = 0;
  REP(i, 0, s.length()) {
    sum = sum * 10 + (s[i] - '0');
    sum %= m;
  }
  return sum;
}

int main(void){
  string ns, ms;
  cin >> ns >> ms;
  ll n, m;
  n = readMod(ns, mod);
  m = readMod(ms, mod - 1);
  ll sum = 1;
  ll cur = n;
  while (m > 0) {
    if (m % 2) {
      sum *= cur;
      sum %= mod;
    }
    cur = cur * cur % mod;
    m /= 2;
  }
  cout << (n == 0 && ms != "0" ? 0 : sum) << endl;
}
