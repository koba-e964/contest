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

bool check(int n, int k, int e) {
  if (n < k) {
    return false;
  }
  int ma, mi;
  if (n == k) {
    ma = k;
    mi = k;
  } else {
    ma = k - 1;
    mi = max(0, 2 * k - n);
  }
  return mi <= e && e <= ma;
}

int main(void){
  int n;
  cin >> n;
  VI a(n);
  int ma = 0, mi = 1e8;
  REP(i, 0, n) {
    cin >> a[i];
    ma = max(ma, a[i]);
    mi = min(mi, a[i]);
  }
  bool ok;
  if (ma - mi >= 2) {
    ok = false;
  }
  else if (ma == mi) {
    ok = check(n, ma, 0) || check(n, ma + 1, n);
  } else {
    int e = 0;
    REP(i, 0, n) {
      if (a[i] != ma) {
	e += 1;
      }
    }
    ok = check(n, ma, e);
  }
  cout << (ok ? "Yes" : "No") << endl;
}
