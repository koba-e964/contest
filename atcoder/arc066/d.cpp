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
const ll mod = 1e9 + 7;

ll sub1(ll u, ll mask) {
  ll oldu = u;
  ll poke = 1;
  int pos = 0;
  REP(i, 0, 64) {
    if (mask & (1LL << i)) {
      continue;
    }
    pos++;
  }
  for (int i = 63; i >= 0; --i) {
    if (mask & (1LL << i)) {
      if (u & (1LL << i)) {
	u = (1LL << i) - 1;
      }
      continue;
    }
    pos--;
    if (u & (1LL << i)) {
      poke += 1LL << pos;
    }
  }
  return poke;
}

ll solve(ll);

ll segm(ll a, ll b) {
  ll res = solve(b) - solve(a - 1);
  res = (res + mod) % mod;
  return res;
}

map<ll, ll> memo;

ll solve(ll n) {
  if (memo.count(n)) {
    return memo[n];
  }
  memo[n] = 0;
  ll &ret = memo[n];
  if (n <= 1) {
    return ret = n + 1;
  }
  ll highest = 1;
  while (n >= highest) { highest *= 2; }
  if (n + 1 == highest) {
    ll sub = solve(n / 2);
    return ret = (3 *sub + mod - 1) % mod;
  }
  highest /= 2; // power of 2, n >= highest
  ll tot = solve(highest - 1);
  tot += segm(2 * highest - 2 - n, highest - 2);
  tot %= mod;
  tot += solve(n - highest);
  tot %= mod;
  if(0) {
    cerr << "solve(" << n << ")=" << tot << endl;
  }
  return ret = tot;
}
int main(void){
  ll n;
  cin >> n;
  ll tot = 0;
  cout << solve(n) << endl;
}
