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


void fail(void) {
  cout << "-1\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int f, a;
  ll b;
  int k;
  cin >> a >> b >> f >> k;
  // O(k) simulation
  ll fuel = b - f;
  if (fuel < 0) {
    fail();
  }
  int cnt = 0;
  bool right = true;
  REP(i, 0, k) {
    ll nec;
    if (right) {
      nec = 2 * (a - f);
    } else {
      nec = 2 * f;
    }
    if (i == k - 1) {
      nec /= 2;
    }
    if (fuel < nec) {
      fuel = b;
      cnt += 1;
    }
    if (b < nec) {
      fail();
    }
    right = !right;
    fuel -= nec;
  }
  cout << cnt << "\n";
}
