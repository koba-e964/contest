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
  ll h;
  cin >> n >> h;
  VL a(n), b(n);
  vector<PI> pool;
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
    pool.push_back(PI(b[i], a[i]));
  }
  sort(pool.rbegin(), pool.rend());
  ll rem = h;
  ll cnt = 0;
  ll ma = 0;

  REP(i, 0, n) {
    ll b = pool[i].first;
    ll a = pool[i].second;
    if (b <= ma) {
      break;
    }
    rem -= b;
    cnt += 1;
    ma = max(ma, a);
    if (rem <= 0) {
      break;
    }
  }
  if (rem > 0) {
    cnt += (rem + ma - 1) / ma;
  }
  cout << cnt << "\n";
}
