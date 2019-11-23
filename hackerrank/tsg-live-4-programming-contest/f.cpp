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
typedef pair<ll, int> PLI;

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y; y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll x;
  cin >> n >> x;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  map<ll, int> cur;
  PI ma(-1, -1);
  REP(i, 0, n) {
    map<ll, int> nxt;
    if (a[i] <= x) {
      nxt[a[i]] = 1;
      ma = max(ma, PI(1, -i));
    }
    for (PLI e: cur) {
      ll nk = e.first / gcd(e.first, a[i]) * a[i];
      if (nk <= x) {
        ll nv = max(nxt[nk], e.second + 1);
        nxt[nk] = nv;
        ma = max(ma, PI(e.second + 1, -(i - e.second)));
      }
    }
    cur = nxt;
  }
  int len = ma.first;
  int st = -ma.second;
  cout << st + 1 << " " << st + len << endl;
}
