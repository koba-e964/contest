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
  int k;
  cin >> k;
  VL a(k);
  REP(i, 0, k) {
    cin >> a[i];
  }
  ll l = 2, r = 2;
  for (int i = k - 1; i >= 0; --i) {
    ll nl = (l + a[i] - 1) / a[i] * a[i];
    ll nr = (r / a[i] + 1) * a[i] - 1;
    if (nl > nr) {
      cout << -1 << endl;
      return 0;
    }
    l = nl;
    r = nr;
  }
  cout << l << " " << r << endl;
}
