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

const ll mod = 1e9 + 7;
ll invmod(ll v) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = v;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

int main(void){
  ll h, w, a, b;
  cin >> h >> w >> a >> b;
  // sum_(i = 0)^(h - a - 1) C(i + b - 1, b - 1) * C(h + w - 2 - b - i, w - 1 - b)
  vector<ll> tbl1(h + w), tbl2(h + w);
  REP(i, 0, b - 1) {
    tbl1[i] = 0;
  }
  ll cur = 1;
  REP(i, b - 1, h + w) {
    tbl1[i] = cur;
    cur = cur * (i + 1) % mod;
    cur = cur * invmod(i - b + 2) % mod;
  }

  REP(i, 0, w - b - 1) {
    tbl2[i] = 0;
  }
  cur = 1;
  REP(i, w - b - 1, h + w) {
    tbl2[i] = cur;
    cur = cur * (i + 1) % mod;
    cur = cur * invmod(i - w + b + 2) % mod;
  }
  ll sum = 0;
  REP(i, 0, h - a) {
    sum += tbl1[i + b - 1] * tbl2[h + w - 2 - b - i];
    sum %= mod;
  }
  cout << sum << endl;
}
