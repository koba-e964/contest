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

const int N = 20;
ll fac[N];
void init(void) {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i;
}

ll tapris(VI &path) {
  int v = 0;
  REP(i, 0, 10) v += path[i];
  ll ans = fac[v];
  REP(i, 0, 10) ans = ans / fac[path[i]];
  return ans;
}

ll subset_solve(const VI &freq, VI &path, int pos) {
  if (pos >= 10) {
    ll ans = tapris(path);
    if (path[0] >= 1) {
      path[0]--;
      ans -= tapris(path);
      path[0]++;
    }
    return ans;
  }
  if (freq[pos] == 0) return subset_solve(freq, path, pos + 1);
  ll tot = 0;
  REP(i, 1, freq[pos] + 1) {
    path[pos] = i;
    tot += subset_solve(freq, path, pos + 1);
  }
  path[pos] = 0;
  return tot;
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  string n;
  cin >> n;
  VI freq(10);
  REP(i, 0, n.size()) {
    freq[n[i] - '0'] += 1;
  }
  VI uu(10);
  cout << subset_solve(freq, uu, 0) << "\n";
}
