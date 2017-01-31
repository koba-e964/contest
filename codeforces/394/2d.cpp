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



int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll l, r;
  cin >> n >> l >> r;
  VL a(n);
  VI p(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }
  VI inv_p(n);
  REP(i, 0, n) {
    inv_p[p[i]] = i;
  }
  ll cur = -1e14;
  VL b(n, -1);
  REP(i, 0, n) {
    int idx = inv_p[i];
    if (r - a[idx] <= cur) {
      cout << -1 << endl;
      return 0;
    }
    ll bx = max(cur + 1 + a[idx], l);
    b[idx] = bx;
    cur = bx - a[idx];
  }
  REP(i, 0, n) {
    cout << b[i] << (i == n - 1 ? "\n" : " ");
  }
}
