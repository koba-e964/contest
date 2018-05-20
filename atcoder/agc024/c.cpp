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
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) {
    if (a[i] > i) {
      cout << -1 << endl;
      return 0;
    }
    if (i >= 1) {
      if (a[i] - a[i - 1] > 1) {
	cout << -1 << endl;
	return 0;
      }
    }
  }
  ll tot = 0;
  for (int i = n - 1; i >= 0; --i) {
    if (i == n - 1 || a[i + 1] - a[i] != 1) {
      // create a mountain
      tot += a[i];
    }
  }
  cout << tot << endl;
}
