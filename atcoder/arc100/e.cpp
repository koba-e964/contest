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

#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 18;
ll a[1 << N], b[1 << N], ans[1 << N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
#if MOCK
  REP(i, 0, 1 << n) a[i] = i;
#else
  REP(i, 0, 1 << n) cin >> a[i];
#endif
  REP(bits, 0, 1 << n) {
    int comp = (1 << n) - 1 - bits;
    int sub = comp;
    while (1) {
      int sup = (1 << n) - 1 - sub;
      ans[sup] = max(ans[sup], b[sup] ? b[sup] + a[bits] : 0);
      b[sup] = max(b[sup], a[bits]);
      if (sub == 0) break;
      sub = (sub - 1) & comp;
    }
  }
  REP(i, 1, 1 << n) {
    ans[i] = max(ans[i], ans[i - 1]);
  }
#if !MOCK
  REP(i, 1, 1 << n) {
    cout << ans[i] << "\n";
  }
#endif
}
