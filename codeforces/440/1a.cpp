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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll solve(ll n) {
  if (n <= 3) {
    return -1;
  }
  int r = n % 4;
  if (r == 0) {
    return n / 4;
  }
  if (r == 1) {
    return n >= 9 ? n / 4 - 1 : -1;
  }
  if (r == 2) {
    return n >= 6 ? n / 4 : -1;
  }
  // r == 3
  return n >= 15 ? n / 4 - 1 : -1;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> q;
  while (q--) {
    ll n;
    cin >> n;
    cout << solve(n) << endl;
  }
}
