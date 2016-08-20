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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 100100;
ll h[N], s[N];
int n;

bool ok(ll x) {
  vector<ll> tol(n);
  REP(i, 0, n) {
    if (x < h[i]) {
      return 0;
    }
    tol[i] = (x - h[i]) / s[i]; // balloon i should be shot by t=tol[i]
  }
  sort(tol.begin(), tol.end());
  REP(i, 0, n) {
    if (tol[i] < i) {
      return 0;
    }
  }
  return 1;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> h[i] >> s[i];
  }
  ll lo = 0;
  ll hi = (ll) n << 32;
  while (hi - lo > 1) {
    ll mid = (lo + hi) / 2;
    if (ok(mid)) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << hi << endl;
}
