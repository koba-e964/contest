#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

map<int, VI> pool;

int main(void) {
  int n;
  ll d, t;
  cin >> n >> d >> t;
  REP (i, 0, n) {
    int x;
    cin >> x;
    int r = (x % d + d) % d;
    if (pool.find(r) == pool.end()) {
      pool[r] = VI();
    }
    pool[r].push_back(x);
  }
  ll ans = 0;
  for (map<int, VI>::iterator it = pool.begin(); it != pool.end(); ++it) {
    VI &s = it->second;
    sort(s.begin(), s.end());
    ans += 2 * t + 1;
    REP (i, 1, s.size()) {
      ans += min(((ll)s[i] - s[i - 1]) / d, 2 * t + 1);
    }
  }
  cout << ans << endl;
}
