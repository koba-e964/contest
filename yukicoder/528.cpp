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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod1 = 1e9;
const ll mod2 = 1e9 + 7;

ll powmod(ll a, ll e, ll mod) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll rex(const string &s, ll mod) {
  int n = s.length();
  ll cur = 0;
  REP(i, 0, n) {
    cur = 10 * cur + (s[i] - '0');
    cur %= mod;
  }
  return cur;
}

ll calc(const string &s, ll mod) {
  int n = s.length();
  if (n % 2 == 1) {
    ll t = rex(s.substr(0, n / 2 + 1), mod);
    t += powmod(10, n / 2, mod) + mod - 1;
    t %= mod;
    string cp(s);
    REP(i, 0, n / 2) {
      cp[n - 1 - i] = s[i];
    }
    if (cp > s) {
      t = (t + mod - 1) % mod;
    }
    return t;
  }
  ll t = rex(s.substr(0, n / 2), mod);
  t += powmod(10, n / 2, mod) + mod - 1;
  t %= mod;
  string cp(s);
  REP(i, 0, n / 2) {
    cp[n - 1 - i] = s[i];
  }
  if (cp > s) {
    t = (t + mod - 1) % mod;
  }
  return t;
}

int main(void){
  string s;
  cin >> s;
  ll t1 = calc(s, mod1);
  ll t2 = calc(s, mod2);
  cout << t1 << endl << t2 << endl;
}
