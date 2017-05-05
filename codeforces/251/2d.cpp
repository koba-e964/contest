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



int main(void){
  int n, m;
  cin >> n >> m;
  VL a(n), b(m);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, m) {
    cin >> b[i];
  }
  sort(a.begin(), a.end());
  sort(b.rbegin(), b.rend());
  if (a[0] >= b[0]) {
    cout << 0 << endl;
    return 0;
  }
  VL a_acc(n + 1, 0), b_acc(m + 1, 0);
  REP(i, 0, n) {
    a_acc[i + 1] = a_acc[i] + a[i];
  }
  REP(i, 0, m) {
    b_acc[i + 1] = b_acc[i] + b[i];
  }
  ll mi = 1e18;
  REP(i, 0, n + m) {
    ll v;
    if (i >= n) {
      v = b[i - n];
    } else {
      v = a[i];
    }
    // Try v
    int ai = upper_bound(a.begin(), a.end(), v) - a.begin();
    int bi = upper_bound(b.begin(), b.end(), v, greater<int>()) - b.begin();
    ll cost = v * ai - a_acc[ai];
    cost += b_acc[bi] - v * bi;
    mi = min(mi, cost);
  }
  cout << mi << endl;
}
