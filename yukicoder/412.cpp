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
  VI r(3);
  int n;
  REP(i, 0, 3) { cin >> r[i]; }
  cin >> n;
  VI e(n);
  REP(i, 0, n) {
    cin >> e[i];
  }
  sort(r.rbegin(), r.rend());
  sort(e.begin(), e.end());
  VI t(3);
  REP(i, 0, 3) {
    t[i] = lower_bound(e.begin(), e.end(), r[i]) - e.begin();
  }
  // t[0] >= t[1] >= t[2]
  VL ans(8);
  REP(bits, 0, 8) {
    int sum;
    int hwid = (bits & 1) ? t[0] : n;
    // ugly hard coding :-(
    int cs = bits >> 1;
    if (cs == 0) {
      sum = 1LL << hwid;
    } else if (cs == 1) {
      sum = (1LL << t[1]) * (hwid - t[1] + 1);
    } else if (cs == 2) {
      int r = hwid - t[2];
      sum = (1LL << t[2]) * (r * (r + 1) / 2 + 1);
    } else {
      assert (cs == 3);
      int r = hwid - t[1];
      int w = t[1] - t[2];
      sum = (1LL << t[2]) * (r * (w + 1) + w * (w + 1) / 2 + 1);
    }
    ans[bits] = sum;
  }
  for (int bits = 7; bits >= 0; --bits) {
    REP(b2, bits + 1, 8) {
      if ((bits & b2) == bits) {
	ans[bits] -= ans[b2];
      }
    }
  }
  cout << ans[0] << endl;
}
