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
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL acc(2 * n + 1);
  REP(i, 0, 2 * n) {
    acc[i + 1] = acc[i] + a[i % n];
  }
  int s, f;
  cin >> s >> f;
  pair<ll, int> ma(-1, -1);
  REP(i, 1, n + 1) {
    ll v = acc[n + f - i] - acc[n + s - i];
    // cerr << v << " " << i << endl;
    ma = max(ma, make_pair(v, -i));
  }
  cout << -ma.second << "\n";
}
