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


// https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93Gallai_theorem
bool is_valid(VI d) {
  int n = d.size();
  ll nc = 0;
  REP(i, 0, n) { nc += d[i]; }
  assert (nc % 2 == 0);
  VL sum(n + 1, 0);
  REP(i, 0, n) {
    sum[i + 1] = sum[i] + d[i];
  }
  ll sd = 0;
  REP(k, 0, n) {
    sd += d[n - 1 - k];
    ll lim = (ll)k * (ll)(k + 1);
    int idx = upper_bound(d.begin(), d.end() - k - 1, k + 1) - d.begin();
    lim += (ll)(n - k - 1 - idx) * (ll)(k + 1);
    lim += sum[idx];
    if (DEBUG) {
      cerr << "k = " << k << endl;
      DEBUGP(sd);
      DEBUGP(lim);
    }
    if (sd > lim) {
      return false;
    }
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI d(n);
  ll nc = 0;
  REP(i, 0, n) {
    cin >> d[i];
    nc += d[i];
  }
  if (nc % 2 == 1) {
    d[0] += 1;
    sort(d.begin(), d.end());
  }
  if (is_valid(d)) {
    cout << (nc % 2 == 0 ? "YES" : "NO") << endl;
  } else {
    cout << "ABSOLUTELY NO" << endl;
  }
}
