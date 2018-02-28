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
  VI a(6);
  int tot = 0;
  REP(i, 0, 6) {
    cin >> a[i];
    tot += a[i];
  }
  bool ok = false;
  REP(bits, 0, 1 << 6) {
    if (__builtin_popcount(bits) != 3) continue;
    int v = 0;
    REP(i, 0, 6) {
      if (bits & 1 << i) v += a[i];
    }
    if (2 * v == tot) ok = true;
  }
  cout << (ok ? "YES" : "NO") << "\n";
}
