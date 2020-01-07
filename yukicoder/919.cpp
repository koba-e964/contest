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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 10000;
const int B = 280;
const int C = (N + B - 1) / B;
ll a[N];
ll cl[C][B];
int n;
const ll inf = 1e14;

void init(void) {
  REP(i, 0, C) {
    REP(j, 0, B) {
      cl[i][j] = i * B + j < n ? a[i * B + j] : inf;
    }
    sort(cl[i], cl[i] + B);
  }
}

int query_naive(ll x, int l, int r) {
  int c = 0;
  REP(i, l, r) if (a[i] <= x) c++;
  return c;
}


// <= x
int query(ll x, int l, int r) {
  int lb = (l + B - 1) / B;
  int rb = r / B;
  if (lb > rb) return query_naive(x, l, r);
  int c = query_naive(x, l, lb * B);
  c += query_naive(x, rb * B, r);
  REP(i, lb, rb) {
    int idx = upper_bound(cl[i], cl[i] + B, x) - cl[i];
    c += idx;
  }
  return c;
}

ll med(int l, int r) {
  int len = r - l;
  int half = len % 2 == 0 ? len / 2 : len / 2 + 1;
  ll fail = -1e9 - 1;
  ll pass = 1e9 + 1;
  while (pass > fail + 1) {
    ll mid = (pass - fail) / 2 + fail;
    if (query(mid, l, r) >= half) pass = mid;
    else fail = mid;
  }
  return pass;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  REP(i, 0, n) cin >> a[i];
  init();
  ll ma = 0;
  if (DEBUG) {
    REP(i, 0, n) {
      REP(j, i + 1, n + 1) {
        cerr << i << " " << j << " med = " << med(i, j) << endl;
      }
    }
  }
  REP(k, 1, n + 1) {
    VL lft(n / k + 1), rgt(n / k + 1);
    VL lft_ma(n / k + 1);
    ll cur = 0;
    REP(i, 0, n / k) {
      lft[i + 1] = med(i * k, i * k + k);
      lft[i + 1] += lft[i];
      cur = max(cur, lft[i + 1]);
      lft_ma[i + 1] = cur;
    }
    REP(i, 0, n / k) {
      rgt[i + 1] = med(n - i * k - k, n - i * k);
      rgt[i + 1] += rgt[i];
    }
    REP(i, 0, n / k + 1) {
      ma = max(ma, k * (rgt[i] + lft_ma[n / k - i]));
    }
    if (DEBUG) {
      cerr << "k = " << k << " " << ma << endl;
    }
  }
  cout << ma << endl;
}
