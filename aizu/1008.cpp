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
  while (cin >> n && n) {
    VI a(n);
    REP(i, 0, n) cin >> a[i];
    int cnt = 0;
    int cur = -1;
    REP(i, 0, n) {
      if (cnt == 0) {
        cur = a[i];
        cnt = 1;
      } else if (cur == a[i]) {
        cnt += 1;
      } else {
        cnt -= 1;
      }
    }
    cnt = 0;
    REP(i, 0, n) cnt += a[i] == cur;
    if (2 * cnt > n) {
      cout << cur << endl;
    } else {
      cout << "NO COLOR" << endl;
    }
  }
}
