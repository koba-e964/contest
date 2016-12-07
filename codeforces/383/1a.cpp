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
const ll mod = 1e9 + 7;

ll gcd(ll x, ll y) {
  return y == 0 ? x : gcd(y, x % y);
}

int main(void){
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  VI per(n);
  VI visited(n, 0);
  REP(i, 0, n) {
    if (visited[i]) {
      continue;
    }
    int v = a[i];
    visited[i] = 1;
    int c = 1;
    while (v != i) {
      if (visited[v] == 1) {
	break;
      }
      visited[v] = 1;
      v = a[v];
      c++;
    }
    if (v != i) {
      cout << -1 << endl;
      return 0;
    }
    per[i] = c;
    v = a[i];
    while (v != i) {
      per[v] = c;
      v = a[v];
    }
  }
  if (0) {
    REP(i, 0, n)
    cerr << "per[" << i << "]=" << per[i] << endl;
  }
  ll sum = 1;
  REP(i, 0, n) {
    ll v = per[i] % 2 == 0 ? per[i] / 2 : per[i];
    sum = sum / gcd(sum, v) * v;
  }
  cout << sum << endl;
}
