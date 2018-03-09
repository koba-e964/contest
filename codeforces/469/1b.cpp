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

#if 0

int main(void) {
  int n;
  cin >> n;
  VI acc(2 * n - 1);
  REP(i, 0, n) {
    acc[2 * i] = i + 1;
  }
  REP(i, 0, n - 1) {
    int idx = -1;
    for (int j = 2 * n - 2 - i; j >= 0; --j) {
      if (acc[j] == 0) {
	idx = j;
	break;
      }
    }
    swap(acc[idx], acc[2 * n - 2 - i]);
  }
  cerr << "result:";
  REP(i, 0, 2 * n - 1) {
    cerr << " " << acc[i];
  }
  cerr << endl;
}

#else

ll calc(ll n, ll x) {
  if (x % 2 == 1) {
    return x / 2 + 1;
  }
  if (n % 2 == 0) {
    return calc(n / 2, x / 2) + n / 2;
  }
  ll idx = x / 2 - 1;
  if (idx == 0) {
    idx = n / 2;
  }
  return calc(n / 2, idx) + (n + 1) / 2;
  
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  int q;
  cin >> n >> q;
  REP(_, 0, q) {
    ll x;
    cin >> x;
    cout << calc(n, x) << endl;
  }
}

#endif
