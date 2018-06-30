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
  VL x(n);
  REP(i, 0, n) cin >> x[i];
  sort(x.begin(), x.end());
  VL acc(n + 1);
  REP(i, 0, n) {
    acc[i + 1] = acc[i] + x[i];
  }
  REP(i, 0, q) {
    ll c, d;
    cin >> c >> d;
    int idx1 = lower_bound(x.begin(), x.end(), c - d) - x.begin();
    int idxc = lower_bound(x.begin(), x.end(), c) - x.begin();
    int idx2 = lower_bound(x.begin(), x.end(), c + d) - x.begin();
    ll ans = (ll) (idx1 + n - idx2) * d;
    ans -= (ll) (idx2 - idxc) * c;
    ans += (ll) (idxc - idx1) * c;
    ans += acc[idx2] - acc[idxc];
    ans -= acc[idxc] - acc[idx1];
    cout << ans << endl;
  }
}
