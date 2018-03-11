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

int lis(const VI &a) {
  int n = a.size();
  // LIS, longest increasing subsequence
  VI m(n + 1);
  int l = 0;
  REP(i, 0, n) {
    int lo = 0;
    int hi = l + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (a[m[mid]] < a[i]) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    int newl = lo + 1;
    m[newl] = i;
    if (newl > l) {
      l = newl;
    }
  }
  return l;
}


#if 0

int main(void) {
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    a[i] = i + 1;
  }
  set<PI> occ;
  do {
    VI b(n);
    REP(i, 0, n) b[i] = -a[i];
    int inc = lis(a);
    int dec = lis(b);
    if (occ.count(PI(inc, dec)) == 0) {
      cout << "a=" << inc << " b=" << dec << endl;
      cout << "elem:";
      REP(i, 0, n) {
	cout << " " << a[i];
      }
      cout << endl;
    }
    occ.insert(PI(inc, dec));
  } while (next_permutation(a.begin(), a.end()));
  for (auto v: occ) {
    cout << "a=" << v.first << " b=" << v.second << endl;
  }
}

#else

VI div(int n, int a, int b) {
  VI ret(a, 1);
  n -= a;
  REP(i, 0, a) {
    int v = min(n, b - 1);
    ret[i] += v;
    n -= v;
  }
  return ret;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n, a, b;
  cin >> n >> a >> b;
  if (a + b - 1 > n || a * b < n) {
    cout << -1 << endl;
    return 0;
  }
  VI d = div(n, a, b);
  VI ans(n);
  int pos = 0;
  REP(i, 0, d.size()) {
    int init = pos + 1;
    REP(j, 0, d[i]) {
      ans[pos++] = init + d[i] - 1 - j;
    }
  }
  assert (lis(ans) == a);
  VI negans(n);
  REP(i, 0, n) negans[i] = -ans[i];
  assert (lis(negans) == b);
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
#endif
