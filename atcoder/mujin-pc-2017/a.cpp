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



int main(void) {
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll prod = 1;
  int pos = 0;
  REP(i, 0, n) {
    if (a[i] < 2 * pos + 1) {
      // solve cut
      prod = prod * ll(pos + 1) % mod;
    } else {
      pos += 1;
    }
  }
  REP(i, 1, pos + 1) {
    prod = prod * i % mod;
  }
  cout << prod << endl;
}
