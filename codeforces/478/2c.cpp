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
  int n, q;
  cin >> n >> q;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VL k(q);
  REP(i, 0, q) cin >> k[i];
  VL acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + a[i];
  ll cur = 0;
  REP(i, 0, q) {
    cur += k[i];
    int cnt = acc.end() - upper_bound(acc.begin(), acc.end(), cur);
    if (cnt == 0) {
      cnt = n;
      cur = 0;
    }
    cout << cnt << "\n";
  }
}
