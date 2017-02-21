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

/*
 * nnode = 0b111...111
 */
ll rec(ll n, ll x, ll nnode) {
  if (x >= nnode) {
    return n;
  }
  if (x <= 0) {
    return 0;
  }
  if (x <= nnode / 2) {
    return rec(n / 2, x, nnode / 2);
  }
  // x >= nnode / 2 + 1 = 0b100...000
  ll sum = n % 2;
  sum += rec(n / 2, x, nnode / 2);
  sum += rec(n / 2, x - nnode / 2 - 1, nnode / 2);
  return sum;
}

ll calc(ll n, ll x) {
  ll cur = 1;
  while (cur <= n) {
    cur *= 2;
  }
  return rec(n, x, cur - 1);
}

int main(void){
  ll n, l, r;
  cin >> n >> l >> r;
  cout << calc(n, r) - calc(n, l - 1) << endl;
}
