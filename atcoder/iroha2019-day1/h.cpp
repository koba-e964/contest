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

ll n;
ll sum;

void dfs(ll x, ll s, ll dig, ll rem) {
  if (s >= sum) {
    if (s == sum && x != n) {
      cout << x << endl;
      exit(0);
    }
    return;
  }
  if (rem * 9 < sum - s) return;
  if (dig == 0) return;
  REP(i, 0, 10) {
    dfs(x + dig * i, s + i, dig / 10, rem - 1);
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;

  ll v = n;
  while (v > 0) {
    sum += v % 10;
    v /= 10;
  }
  ll dig = 1e17;
  dfs(0, 0, dig, 18);
}
