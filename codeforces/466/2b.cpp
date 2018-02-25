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
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n, k, a, b;
  cin >> n >> k >> a >> b;
  if (k == 1) {
    cout << (n - 1) * a << "\n";
    return 0;
  }
  ll tot = 0;
  vector<PL> t;
  while (n > 1) {
    t.push_back(PL(n, tot));
    ll r = n % k;
    tot += a * r;
    n = (n - r) / k;
    tot += b;
  }
  t.push_back(PL(n, tot));
  ll mi = 1e18;
  REP(i, 0, t.size()) {
    PL x = t[i];
    mi = min(a * (x.first - 1) + x.second, mi);
  }
  cout << mi << endl;
}
