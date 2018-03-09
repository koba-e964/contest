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
typedef pair<PI, ll> PPIL;

PI get_intv(int n, int d, int x, int i) {
  int lo = i / (d + 1);
  int hi = n - 1 - (n - 1 - i) / (d + 1);
  if (DEBUG) {
    cerr << "get_intv:" << "[1, " << n <<"] speed=" << d << " idx=" << i <<
      " -> " << "[" << lo <<"," << hi << "]" << endl;
  }
  return PI(lo, hi);
}

bool ok(int n, int d, ll b, const VL &a, int x) {
  if (DEBUG && 0) {
    cerr << "n,d,b="<<n<<","<<d<<","<<b<<endl;
    cerr << "a:";
    REP(i, 0, n){
      cerr << " " << a[i];
    }
    cerr << endl;
    cerr << "x = " << x << endl;
  }
  VL risou(n);
  REP(i, 0, n) {
    risou[i] = min(i, n - 1 - i) < x ? 0 : b;
  }
  if (DEBUG && 0) {
    cerr << "risou:";
    REP(i, 0, n) cerr << " "<< risou[i];
    cerr << endl;
  }
  deque<PPIL> iv;
  REP(i, 0, n) {
    iv.push_back(PPIL(get_intv(n, d, x, i), a[i]));
  }
  VL gen(n);
  int pos = 0;
  while (not iv.empty()) {
    if (pos < n && gen[pos] >= risou[pos]) {
      pos++;
    }
    PPIL top = iv.front(); iv.pop_front();
    int e = top.first.first;
    int f = top.first.second;
    ll g = top.second;
    if (0 && DEBUG) {
      cerr << "interval [" << e << ", " << f << "] (" << g << ")" << endl;
      cerr << "pos = " << pos << endl;
      cerr << "gen:";
      REP(i, 0, n) cerr << " " << gen[i];
      cerr << endl << endl;
    }
    if (g == 0) continue;
    if (pos < e) {
      pos = e;
    }
    if (f < pos) {
      continue;
    }
    if (pos >= n) break;
    if (gen[pos] < risou[pos]) {
      ll dif = risou[pos] - gen[pos];
      dif = min(dif, g);
      gen[pos] += dif;
      g -= dif;
    }
    if (g > 0) {
      iv.push_front(PPIL(PI(e, f), g));
    }
  }
  REP(i, 0, n) {
    if (gen[i] < risou[i]) return false;
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, d;
  ll b;
  cin >> n >> d >> b;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  int pass = (n + 1) / 2, fail = -1;
  while (pass - fail > 1) {
    int mid = (pass + fail) / 2;
    if (ok(n, d, b, a, mid)) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << pass << "\n";
}
