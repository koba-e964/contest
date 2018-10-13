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
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n), b(n);
  ll asum = 0, bsum = 0;
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
    asum += a[i];
    bsum += b[i];
  }
  vector<PL> pool(n);
  REP(i, 0, n) {
    ll x = a[i], y = b[i];
    if (x > y) swap(x, y);
    pool[i] = PL(x, y);
  }
  sort(pool.begin(), pool.end());
  vector<PL> det(n);
  REP(i, 0, n) det[i] = PL(pool[i].second, i);
  sort(det.begin(), det.end());
  ll mi = min(asum, bsum);
  ll tmp = 0;
  REP(i, 0, n) {
    tmp += pool[i].first;
  }
  vector<PL> u(2 * n);
  REP(i, 0, n) {
    u[i] = PL(a[i], i);
    u[i + n] = PL(b[i], i);
  }
  sort(u.begin(), u.end());
  vector<PI> idx(n, PI(-1, -1));
  REP(i, 0, 2 * n) {
    int q = u[i].second;
    if (idx[q].first == -1) idx[q].first = i;
    else idx[q].second = i;
  }
  ll sum = 0;
  REP(i, 0, n) sum += u[i].first;
  REP(i, 0, n) {
    int e = idx[i].first;
    int f = idx[i].second;
    ll val = sum;
    int pos = n;
    if (f >= pos) {
      pos--;
      val += u[f].first;
    }
    if (e >= pos) {
      pos--;
      val += u[e].first;
    }
    /*
    DEBUGP(i);
    DEBUGP(val);
    DEBUGP(e);
    DEBUGP(f);
    */
    REP(i, pos, n) val -= u[i].first;
    mi = min(mi, val);
  }
  /*
  int pos = 0;
  int cnt3 = 0;
  VI occ(n, 1);
  REP(i, 1, n + 1) {
    tmp -= pool[n - i].first;
    occ[n - i] &= ~1;
    if (occ[n - i] == 2) cnt3--;
    // if (occ[n - i] != 0) break;
    int idx = -1;
    while (pos < n) {
      idx = det[pos].second;
      if (cnt3 > 0 || occ[idx] != 0) break;
      pos++;
    }
    if (pos >= n) break;
    pos++;
    occ[idx] |= 2;
    if (occ[idx] == 3) cnt3++;
    tmp += det[i - 1].first;
    if (cnt3 > 0) {
      mi = min(mi, tmp);
    }
    if (1) {
      REP(i, 0, n) cerr << " " << occ[i];
      cerr << endl;
    }
  }
  */
  cout << mi << endl;
}
