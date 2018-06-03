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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll d;
  cin >> n >> d;
  VL x(n);
  REP(i, 0, n) cin >> x[i];
  VI r(n), l(n);
  REP(i, 0, n) {
    // [l[i], r[i])
    r[i] = upper_bound(x.begin() + i, x.end(), x[i] + d) - x.begin();
    l[i] = lower_bound(x.begin(), x.begin() + i + 1, x[i] - d) - x.begin();
  }
  VL acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + r[i];
  ll tot = 0;
  REP(j, 0, n) {
    ll i = l[j];
    ll ulim = r[j];
    tot += (j - i) * ulim - (acc[j] - acc[i]);
    if (0) {
      DEBUGP(j);
      DEBUGP(tot);
    }
  }
  cout << tot << endl;
}
