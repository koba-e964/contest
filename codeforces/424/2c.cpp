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
  int k, n;
  cin >> k >> n;
  VI a(k), b(n);
  set<int> sa;
  REP(i, 0, k) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  REP(i, 0, k - 1) {
    a[i + 1] += a[i];
  }
  REP(i, 0, k) {
    sa.insert(a[i]);
  }
  sort(b.begin(), b.end());
  int tot = 0;
  for (int elem: sa) {
    int delta = elem - b[0];
    bool ok = true;
    REP(j, 0, n) {
      if (sa.count(b[j] + delta) == 0) {
	ok = false;
	break;
      }
    }
    if (ok) {
      tot += 1;
    }
  }
  cout << tot << "\n";
}
