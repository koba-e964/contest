#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll calc(ll a) {
  ll sum = 0;
  while (a > 0) {
    sum += a % 10;
    a /= 10;
  }
  return sum;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  ll ma = 0;
  ll cur = 1;
  while (cur <= n) {
    ll v = (n + 1) / cur * cur - 1;
    ma = max(ma, calc(v));
    cur *= 10;
  }
  cout << ma << "\n";
}
