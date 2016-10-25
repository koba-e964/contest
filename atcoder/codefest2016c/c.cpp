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
  cin >> n;
  VL t(n), a(n);
  REP(i, 0, n) {
    cin >> t[i];
  }
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL def(n, -1);
  bool ok = 1;
  def[0] = t[0];
  REP(i, 1, n) {
    if (t[i - 1] < t[i]) {
      def[i] = t[i];
    }
  }
  if (def[n - 1] >= 0) {
    if (def[n - 1] != a[n - 1]) {
      ok = 0;
    }
  } else {
    def[n - 1] = a[n - 1];
  }
  REP(i, 0, n - 1) {
    if (a[i] > a[i + 1]) {
      if (def[i] >= 0) {
	if (def[i] != a[i]) {
	  ok = 0;
	}
      } else {
	def[i] = a[i];
      }
    }
  }
  REP(i, 0, n) {
    if (def[i] >= 0 && (def[i] > t[i] || def[i] > a[i])) {
      ok = 0;
    }
  }
  if (!ok) {
    cout << 0 << endl;
    return 0;
  }
  ll sum = 1;
  REP(i, 0, n) {
    if (def[i] == -1) {
      sum *= min(t[i], a[i]);
      sum %= mod;
    }
  }
  cout << sum << endl;
}
