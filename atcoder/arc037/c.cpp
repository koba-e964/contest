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

const int N = 3e4 + 10;
int n;
ll k;
ll a[N], b[N];

ll calc(ll v) {
  ll sum = 0;
  REP(i, 0, n) {
    sum += upper_bound(b, b + n, v / a[i]) - b;
  }
  return sum;
}

int main(void){
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  sort(a, a + n);
  sort(b, b + n);
  ll lo = 0;
  ll hi = 1LL << 61;
  while (hi - lo > 1) {
    ll mid = (lo + hi) / 2;
    if (calc(mid) >= k) hi = mid;
    else lo = mid;
  }
  cout << hi << endl;
}
