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
  int n;
  ll a, b;
  cin >> n >> a >> b;
  VL h(n);
  REP(i, 0, n) {
    cin >> h[i];
  }
  ll lo = 0;
  ll hi = 2e9 / b;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    // check
    VL u(n);
    REP(i, 0, n) {
      u[i] = h[i] - b * mid;
    }
    ll nc = 0;
    REP(i, 0, n) {
      if (u[i] >= 0) {
	nc += (u[i] + (a - b - 1)) / (a - b);
      }
    }
    if (nc <= mid) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << hi << endl;
}
