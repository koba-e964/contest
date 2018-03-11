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
  ll n, k;
  cin >> n >> k;
  ll tot = 0;
  REP(b, k + 1, n + 1) {
    ll qmax = (n - k) / b;
    tot += (b - k) * qmax;
    ll r = (n - k) % b;
    tot += min(b - k, r + 1);
    if (k == 0) {
      tot--;
    }
    if (0) {
      DEBUGP(b);
      DEBUGP(qmax);
      DEBUGP(r);
      DEBUGP(tot);
    }
  }
  cout << tot << endl;
}
