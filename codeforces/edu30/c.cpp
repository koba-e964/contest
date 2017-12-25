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
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  vector<VI> a(m, VI(n, 0)); // TRANSPOSED!
  REP(i, 0, n) {
    REP(j, 0, m) {
      cin >> a[j][i];
    }
  }
  int tot = 0;
  int mi = 0;
  REP(i, 0, m) {
    int cnt = 0;
    int ma = 0;
    int mami = 0;
    REP(j, 0, n) {
      int ones = count(a[i].begin() + j, a[i].begin() + j + min(k, n - j), 1);
      if (ma < ones) {
	ma = ones;
        mami = cnt;
      }
      if (a[i][j] == 1) { cnt += 1; }
    }
    tot += ma;
    mi += mami;
  }
  cout << tot << " " << mi << "\n";
}
