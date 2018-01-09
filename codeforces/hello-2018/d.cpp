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

const int N = 243700;
int a[N], t[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll tt;
  cin >> n >> tt;
  
  REP(i, 0, n) {
    cin >> a[i] >> t[i];
  }
  int pass = 0;
  int fail = n + 1;
  while (fail - pass > 1) {
    int mid = (fail + pass) / 2;
    VI pool;
    REP(i, 0, n) {
      if (a[i] >= mid) {
	pool.push_back(t[i]);
      }
    }
    sort(pool.begin(), pool.end());
    bool ok = false;
    if (pool.size() >= mid) {
      ll tot = 0;
      REP(i, 0, mid) {
	tot += pool[i];
      }
      if (tot <= tt) {
	ok = true;
      }
    }
    if (ok) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << pass << "\n";
  cout << pass << "\n";
  {
    vector<PI> pool;
    REP(i, 0, n) {
      if (a[i] >= pass) {
	pool.push_back(PI(t[i], i + 1));
      }
    }
    sort(pool.begin(), pool.end());
    REP(i, 0, pass) {
      cout << pool[i].second << (i == pass - 1 ? "" : " ");
    }
    cout << "\n";
  }
}
