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
  string b;
  cin >> b;
  ll l = -1e9;
  ll r = 1e9;
  REP(i, 4, n) {
    string sub = b.substr(i - 4, 5);
    if (sub == "00001") {
      ll ma = -1e18;
      REP(j, i - 4, i + 1) {
	ma = max(ma, a[j]);
      }
      l = max(l, ma + 1);
    }
    if (sub == "11110") {
      ll mi = 1e18;
      REP(j, i - 4, i + 1) {
	mi = min(mi, a[j]);
      }
      r = min(r, mi - 1);
    }
  }
  cout << l << " " << r << "\n";
}
