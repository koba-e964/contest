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
  int n, k;
  cin >> n >> k;
  VL c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  vector<PI> pool(n);
  REP(i, 0, n) {
    pool[i]=PI(c[i],i);
  }
  sort(pool.rbegin(),pool.rend());
  ll tot = 0;
  VI ans(n);
  set<int> time;
  REP(i, 0, n) {
    time.insert(k + i + 1);
  }
  REP(i, 0, n) {
    int idx = pool[i].second;
    int t = idx + 1;
    int u = *time.lower_bound(t);
    ans[idx] = u;
    time.erase(u);
    tot += c[idx] * (u - idx - 1);
  }
  cout << tot << "\n";
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
