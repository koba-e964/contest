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
  int n, m, k;
  cin >> n >> m >> k;
  VI pop(n, 1);
  REP(i, 0, m) {
    int x;
    cin >> x;
    pop[x] = 0;
  }
  VI a(k);
  REP(i, 0, k) cin >> a[i];
  VI pre(n, -1);
  {
    int sigma = -1;
    REP(i, 0, n) {
      if (pop[i]) sigma = i;
      pre[i] = sigma;
    }
  }
  ll mi = 1e16;
  REP(yuki, 1, k + 1) {
    int num = 0;
    int cur = pre[0];
    while (cur < n) {
      int nxt = cur + yuki >= n ? n : pre[cur + yuki];
      if (cur <= -1 || nxt <= cur) {
        num = -1;
        break;
      }
      cur = nxt;
      num++;
    }
    if (0) {
      DEBUGP(yuki);
      DEBUGP(num);
    }
    if (num >= 0) {
      mi = min(mi, (ll) a[yuki - 1] * (ll) num);
    }
  }
  cout << (mi == 1e16 ? -1 : mi) << "\n";
}
