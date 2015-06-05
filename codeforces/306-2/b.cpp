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
const int N = 17;
int n;
ll l, r, x;
ll c[N];
int main(void){
  cin >> n >> l >> r >> x;
  REP (i, 0, n) {
    cin >> c[i];
  }
  int cnt = 0;
  REP (bits, 1, 1 << n) {
    if ((bits & (-bits)) == bits) {
      continue;
    }
    int s = 0;
    ll ma = 0, mi = 1LL << 32;
    REP(i, 0, n) {
      if (bits & (1 << i)) {
	s += c[i];
	ma = max(ma , c[i]);
	mi = min(mi, c[i]);
      }
    }
    if (s >= l && s <= r && ma - mi >= x) {
      cnt ++;
    }
  }
  cout << cnt << endl;
}
