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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  ll n, x, s;
  int m, k;
  cin >> n >> m >> k >> x >> s;
  VL a(m + 1), b(m + 1), c(k), d(k);
  REP(i, 0, m) {
    cin >> a[i];
  }
  REP(i, 0, m) {
    cin >> b[i];
  }
  REP(i, 0, k) {
    cin >> c[i];
  }
  REP(i, 0, k) {
    cin >> d[i];
  }
  ll mi = n * x;
  a[m] = x;
  b[m] = 0;
  REP(i, 0, m + 1) {
    if (s < b[i]) { continue; }
    ll rem = s - b[i];
    int idx = upper_bound(d.begin(), d.end(), rem) - d.begin();
    if (idx > 0) {
      mi = min(mi, max(0LL, n - c[idx - 1]) * a[i]);
    } else {
      mi = min(mi, n * a[i]);
    }
  }
  cout << mi << endl;
}
