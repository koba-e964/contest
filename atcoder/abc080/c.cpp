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
  VI f(n);
  vector<VI> p(n, VI(11));
  REP(i, 0, n) {
    REP(j, 0, 10) {
      int x;
      cin >> x;
      f[i] |= x << j;
    }
  }
  REP(i, 0, n) {
    REP(j, 0, 11) {
      cin >> p[i][j];
    }
  }
  ll ma = -1e16;
  REP(bits, 1, 1 << 10) {
    ll tot = 0;
    REP(i, 0, n) {
      tot += p[i][__builtin_popcount(f[i] & bits)];
    }
    ma = max(ma, tot);
  }
  cout << ma << endl;
}
