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

void fail(void) {
  cout << -1 << endl;
  exit(0);
}

int main(void){
  int n;
  ll w;
  cin >> n>> w;
  vector<pair<ll, int> > a(n);
  REP(i, 0, n) {
    ll t;
    cin >> t;
    a[i] = make_pair(t, i);
  }
  sort(a.rbegin(), a.rend());
  VL cap(n);
  REP(i, 0, n) {
    ll t = (a[i].first + 1) / 2;
    cap[i] = a[i].first - t;
    w -= t;
  }
  if (w < 0) {
    fail();
  }
  int pos = 0;
  while (pos < n && w > 0) {
    if (cap[pos] > 0) {
      ll t = min(w, cap[pos]);
      cap[pos] -= t;
      w -= t;
    }
    pos += 1;
  }
  if (w > 0) {
    fail();
  }
  VL ret(n);
  REP(i, 0, n) {
    int p = a[i].second;
    ret[p] = a[i].first - cap[i];
  }
  REP(i, 0, n) {
    cout << ret[i] << (i == n - 1 ? "\n" : " ");
  }
}
