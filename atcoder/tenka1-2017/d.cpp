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

const int N = 123456;
ll a[N], b[N];

int main(void) {
  int n;
  ll k;
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
  }
  ll ma = 0;
  k += 1;
  while (k > 0) {
    ll kk = k - 1;
    ll tot = 0;
    REP(i, 0, n) {
      if ((a[i] & kk) == a[i]) {
	tot += b[i];
      }
    }
    ma = max(ma, tot);
    k &= k - 1;
  }
  cout << ma << endl;
}
