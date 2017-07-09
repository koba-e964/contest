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
    cin >> n;
    const int BIAS = 1425;
    VI height(2 * BIAS, 0);
    ll tot = 0;
    REP(i, 0, n) {
      ll p, c;
      cin >> p >> c;
      assert (p >= -BIAS && p < BIAS);
      p += BIAS;
      ll cost = 1;
      height[p] += c;
      REP(i, p + 1, 2 * BIAS) {
	if (c * (height[i] - height[i - 1]) < -1) {
	  height[i] += c;
	  cost += 1;
	} else {
	  break;
	}
      }
      for (int i = p - 1; i >= 0; --i) {
	if (c * (height[i] - height[i + 1]) < -1) {
	  height[i] += c;
	  cost += 1;
	} else {
	  break;
	}
      }
      tot += cost * (ll(i) + 1);
    }
    cout << tot << endl;
  }
}
