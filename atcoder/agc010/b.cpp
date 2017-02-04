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

void fail(void) {
  cout << "NO" << endl;
  exit(0);
}

int main(void){
  int n;
  cin >> n;
  VL a(n);
  ll tot = 0;
  REP(i, 0, n) {
    cin >> a[i];
    tot += a[i];
  }
  ll cum = (ll(n) * ll(n + 1)) / 2;
  if (tot % cum != 0) {
    fail();
  }
  ll cnt = tot / cum;
  REP(i, 0, n) {
    a[i] -= cnt * i;
  }
  ll ksum = a[0];
  REP(i, 0, n) {
    a[i] -= ksum;
  }
  if (0) {
    REP(i, 0, n) {
      cerr << "a[" << i << "]=" << a[i] << endl;
    }
  }
  REP(i, 0, n) {
    if (a[i] % n != 0) {
      fail();
    }
    if (i < n - 1) {
      if (a[i] < a[i + 1]) {
	fail();
      }
    }
  }
  cout << "YES" << endl;
}
