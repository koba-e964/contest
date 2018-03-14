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
  cin >> n;
  VL v(n), t(n);
  REP(i, 0, n) cin >> v[i];
  REP(i, 0, n) cin >> t[i];
  VL acc(n + 1, 0);
  REP(i, 0, n) acc[i + 1] = acc[i] + t[i];
  VL imos(n + 1);
  VL ans(n + 1);
  REP(i, 0, n) {
    // When does the pile i disappear?
    int idx = upper_bound(acc.begin() + i, acc.end(), acc[i] + v[i])
      - acc.begin() - 1;
    ll pre = acc[idx] - acc[i];
    ans[idx] += v[i] - pre;
    imos[i] += 1;
    imos[idx] -= 1;
  }
  REP(i, 1, n + 1) {
    imos[i] += imos[i - 1];
  }
  REP(i, 0, n) {
    ans[i] += imos[i] * t[i];
  }
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
