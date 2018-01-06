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
#include <unordered_map>
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

const ll lim = 1e18;
const int N = 16;
const int B = 8;
int n;
int p[N];

ll reccall2;
VL smooth;

void sub2(ll x, int bits, int m) {
  reccall2++;
  smooth.push_back(x);
  int ma = 0;
  REP(i, 0, m) {
    if (bits & 1 << i) {
      ma = i;
    }
  }
  REP(i, ma, m) {
    if (x <= lim / p[i]) {
      int nvec = bits;
      nvec |= 1 << i;
      sub2(x * p[i], nvec, m);
    }
  }
}

void enum_smooth(int m) {
  sub2(1, 0, m);
}

ll reccall;

ll sub(ll x, int idx) {
  reccall++;
  if (idx == B) {
    int pos = upper_bound(smooth.begin(), smooth.end(), x) - smooth.begin();
    return pos;
  }
  idx--;
  ll tot = 0;
  while (x > 0) {
    tot += sub(x, idx);
    x /= p[idx];
  }
  return tot;
}


ll smooth_count(ll x) {
  return sub(x, n);
}



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  REP(i, 0, n) {
    cin >> p[i];
  }
  sort(p, p + n);
  ll k;
  cin >> k;
  if (n <= B) {
    enum_smooth(n);
    sort(smooth.begin(), smooth.end());
    cout << smooth[k - 1] << "\n";
    return 0;
  }
  enum_smooth(B);
  sort(smooth.begin(), smooth.end());
  ll pass = 0;
  ll fail = lim + 1;
  while (fail - pass > 1) {
    ll mid = (pass + fail) / 2;
    if (smooth_count(mid) <= k - 1) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << fail << "\n";
}
