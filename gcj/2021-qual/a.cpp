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

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int n;
    cin >> n;
    int ans = 0;
    VI l(n);
    REP(i, 0, n) cin >> l[i], l[i]--;
    REP(i, 0, n - 1) {
      PI mi(l[i], i);
      REP(j, i + 1, n) mi = min(mi, PI(l[j], j));
      int idx = mi.second;
      ans += idx - i + 1;
      reverse(l.begin() + i, l.begin() + idx + 1);
    }
    cout << "Case #" << case_nr << ": " << ans << "\n";
  }
}
