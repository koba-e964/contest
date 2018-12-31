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

const int DEBUG = 0;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll l;
  int n;
  cin >> l >> n;
  VL x(n);
  REP(i, 0, n) cin >> x[i];
  VL accl(n + 1, 0), accr(n + 1, 0);
  REP(i, 0, n) accl[i + 1] = accl[i] + x[i];
  REP(i, 0, n) accr[i + 1] = accr[i] + (l - x[n - 1 - i]);
  if (DEBUG) {
    REP(i, 0, n + 1) cerr << " " << accl[i];
    cerr << endl;
    REP(i, 0, n + 1) cerr << " " << accr[i];
    cerr << endl;
  }
  ll ma = max(x[n - 1], l - x[0]);
  REP(i, 0, n - 1) {
    int cnt = min(i, n - 2 - i);
    int cntl = i > cnt ? cnt + 1 : cnt;
    ll tmp = 2 * (accr[n - 1 - i] - accr[n - 1 - i - cnt - 1] + accl[i] - accl[i - cntl]) + x[i];
    if (DEBUG) {
      DEBUGP(i);
      DEBUGP(cnt);
      DEBUGP(tmp);
    }
    ma = max(ma, tmp);
  }
  REP(i, 0, n - 1) {
    int cnt = min(i, n - 2 - i);
    int cntl = i > cnt ? cnt + 1 : cnt;
    ll tmp = 2 * (accl[n - 1 - i] - accl[n - 1 - i - cnt - 1] + accr[i] - accr[i - cntl]) + (l - x[n - 1 - i]);
    if (DEBUG) {
      DEBUGP(i);
      DEBUGP(tmp);
    }
    ma = max(ma, tmp);
  }
  cout << ma << endl;
}
