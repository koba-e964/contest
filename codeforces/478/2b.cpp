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
  VL a(14);
  REP(i, 0, 14) cin >> a[i];
  ll ma = 0;
  REP(i, 0, 14) {
    if (a[i] == 0) continue;
    VL b(a);
    ll q = a[i] / 14;
    ll r = a[i] % 14;
    b[i] = 0;
    REP(j, 0, 14) b[j] += q;
    REP(j, 0, r) b[(i + j + 1) % 14]++;
    ll tot = 0;
    REP(j, 0, 14) if (b[j] % 2 == 0) tot += b[j];
    ma = max(ma, tot);
  }
  cout << ma << endl;
}
