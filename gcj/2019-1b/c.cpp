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


// I don't want to implement tedious O(n log n) solution
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int n, k;
    cin >> n >> k;
    VI c(n), d(n);
    REP(i, 0, n) cin >> c[i];
    REP(i, 0, n) cin >> d[i];
    ll ans = 0;
    REP(i, 0, n) {
      int mac = 0;
      int mad = 0;
      REP(j, i, n) {
        mac = max(mac, c[j]);
        mad = max(mad, d[j]);
        if (abs(mac - mad) <= k) ans += 1;
      }
    }
    cout << "Case #" << case_nr << ": " << ans << endl;
  }
}
