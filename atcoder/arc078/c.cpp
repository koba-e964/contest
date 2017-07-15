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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  ll atot = 0;
  REP(i, 0, n) {
    cin >> a[i];
    atot += a[i];
  }
  ll cur = 0;
  ll mi = 5e15;
  REP(i, 0, n) {
    if (i == n - 1) {
      break;
    }
    cur += a[i];
    ll diff = abs(cur + cur - atot);
    mi = min(mi, diff);
  }
  cout << mi << "\n";
}
