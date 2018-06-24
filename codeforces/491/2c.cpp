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
  ll n;
  cin >> n;
  ll pass = n, fail = 0;
  while (pass - fail > 1) {
    ll mid = (pass + fail) / 2;
    ll rem = n;
    ll picked = 0;
    while (rem > 0) {
      ll x = min(rem, mid);
      picked += x;
      rem -= x;
      rem -= rem / 10;
    }
    if (picked * 2 >= n) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << pass << "\n";
}
