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
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, int> PLI;
const ll mod = 1e9 + 7;

const int N = 100010;
ll a[N];

ll mabs(ll x) {
  return max(x, -x);
}

pair<ll, PI> solve(ll t, const vector<PLI> &pool) {
  ll mi = mabs(a[0]);
  PI mini = PI(0, 1);
  int pos = 0;
  ll tot = 0;
  int n = pool.size();
  REP(i, 0, n - 1) {
    if (pos == i) {
      pos = i + 1;
      tot = pool[i + 1].first - pool[i].first;
    }
    while (pos < n) {
      if (mabs(t - tot) < mabs(t - mi)) {
	mi = tot;
	mini = PI(pool[i].second, pool[pos].second);
      }
      if (pos == n - 1 || tot > t) {
	break;
      }
      pos++;
      tot += pool[pos].first - pool[pos - 1].first;
    }
    tot -= pool[i + 1].first - pool[i].first;
  }
  if (mini.second < mini.first) {
    swap(mini.first, mini.second);
  }
  return pair<ll, PI>(mi, mini);
}


int main(void){
  int n, k;
  while (scanf("%d%d", &n, &k)) {
    if (n == 0 && k == 0) {
      break;
    }

    REP(i, 0, n) {
      scanf("%lld", a + i);
    }
    vector<PLI> pool;
    ll tot = 0;
    REP(i, 0, n) {
      tot += a[i];
      pool.push_back(PLI(tot, i + 1));
    }
    pool.push_back(PLI(0, 0));
    sort(pool.begin(), pool.end());

    REP(i, 0, k) {
      ll t;
      scanf("%lld", &t);
      pair<ll, PI> res1 = solve(t, pool);
      cout << res1.first << " " << res1.second.first + 1 << " " << res1.second.second << endl;
    }
  }
  
}
