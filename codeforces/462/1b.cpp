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
  ll p, k;
  cin >> p >> k;
  VL ans;
  while (p != 0) {
    ll r = p % k;
    if (r < 0) { r += k; }
    ans.push_back(r);
    p = (p - r) / -k;
  }
  cout << ans.size() << "\n";
  REP(i, 0, ans.size()) {
    cout << ans[i] << (i == (int)ans.size() - 1 ? "\n" : " ");
  }
}
