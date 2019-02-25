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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

const int N = 200000;

int n;

ll a[N], d[N]; // b[i] = a[i] + d[i]
ll asum;

set<PL> neg, pos;
ll possum;

void add_d(int x, ll y) {
  d[x] = y;
  if (x == 0 || x == 2 * n - 1) return;
  if (y >= 0) {
    possum += y;
    pos.insert(PL(y, x));
  } else {
    neg.insert(PL(y, x));
  }
}

void rm_d(int x) {
  ll y = d[x];
  if (x == 0 || x == 2 * n - 1) return;
  if (y >= 0) {
    possum -= y;
    pos.erase(PL(y, x));
  } else {
    neg.erase(PL(y, x));
  }
}

ll solve() {
  ll tot = possum;
  if (pos.size() % 2 == 1) {
    tot += max(-pos.begin()->first, neg.rbegin()->first);
  }
  return tot + asum;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> n >> q;
  VL a(2 * n), b(2 * n);
  REP(i, 0, 2 * n) {
    cin >> a[i];
    asum += a[i];
  }
  REP(i, 0, 2 * n) {
    ll b;
    cin >> b;
    add_d(i, b - a[i]);
  }
  REP(i, 0, q) {
    int p, x, y;
    cin >> p >> x >> y;
    p--;
    asum += x - a[p];
    a[p] = x;
    rm_d(p);
    add_d(p, y - x);
    cout << solve() << "\n";
  }
}
