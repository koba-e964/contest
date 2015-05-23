#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
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

const ll mod = 1e9+7;

ll invmod(ll x) {
  ll sum = 1;
  ll cur = x;
  ll n = mod - 2;
  while(n > 0) {
    if (n % 2) {
      sum = sum * cur % mod;
    }cur = cur * cur % mod;
    n /= 2;
  }
  return sum;
}


int main(void){
  ll a,b,c;
  cin >> a >> b >> c;
  ll denom = (b * a + c * a) % mod;
  denom -= b * c % mod;
  denom += mod;
  denom %= mod;
  ll dinv = invmod(denom);
  ll cc = (a * c % mod) * dinv % mod;
  cc = cc + mod - 1;
  cc %= mod;
  ll rr = (a * b % mod) * dinv % mod;
  rr = rr + mod - 1;
  rr %= mod;
  cout << rr << " " << cc << endl;
}
