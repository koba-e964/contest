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
  int t;
  cin >> t;
  while (t--) {
    int n;
    ll x, a, b, m;
    cin >> n >> x >> a >> b >> m;
    VI r(n);
    r[0] = x;
    REP(i, 1, n) {
      r[i] = ((r[i - 1] ^ a) + b) % m;
    }
    REP(i, 0, n) {
      r[i] %= 10;
    }
    int tot = 0;
    int cur = 1;
    int exp3 = 0;
    bool all_zero = 1;
    REP(i, 0, n) {
      int u = r[i];
      if (u != 0) {
	all_zero = false;
      }
      tot += exp3 >= 2 ? 0 : exp3 == 1 ? (u * cur * 3) : u * cur;
      tot %= 9;
      if (i == n - 1) {
	break;
      }
      int v = n - 1 - i;
      int w = i + 1;
      while (v % 3 == 0) {
	v /= 3;
	exp3++;
      }
      while (w % 3 == 0) {
	w /= 3;
	exp3--;
      }
      assert (exp3 >= 0);
      cur *= v;
      cur %= 9;
      w %= 9;
      cur *= w * w * w * w * w;
      cur %= 9;
    }
    cout << (all_zero ? 0 : 1 + (tot + 8) % 9) << endl;
  }
}
