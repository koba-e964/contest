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
  int t;
  cin >> t;
  while (t--) {
    ll x;
    cin >> x;
    PI sol(-1, -1);
    for (ll i=1; i*i < x; ++i){
      if (x%i != 0) continue;
      ll j = x / i;
      if ((i + j) % 2 != 0) continue;
      ll n = (i + j) / 2;
      ll k = (j - i) / 2;
      ll m = n / k;
      if (n / m != k) continue;
      // found
      sol = PI(n, m);
      break;
    }
    if (x == 0) {
      sol = PI(1, 1);
    }
    if (sol.first == -1) {
      cout << "-1\n";
    } else {
      cout << sol.first << " " << sol.second << "\n";
    }
  }
}
