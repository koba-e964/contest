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

ll f(ll w, ll h) {
  return h > w ? (2 * h - w + 1) * w / 2 : w + h * (h - 1) / 2;
}

int main(void){
  ll n, m, k;
  cin >> n >> m >> k;
  int d = 1;
  ll lo = 1;
  ll hi = m - n + 2;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    if (f(k - 1, mid - 1) + mid + f(n - k, mid - 1) > m) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << lo << endl;
}
