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
const ll mod = 1e9 + 7;

const int N = 1e5 + 10;
int a[N], b[N];

ll f(ll a, int b, int k) {
  ll p = 1;
  REP(i, 0, k - 1) p *= 10;
  ll s = (10 * p - 1) / a + 1;
  s -= ((b + 1) * p + a - 1) / a;
  s += (b * p + a - 1) / a;
  return s;
}

int main(void){
  int n, k;
  ios::sync_with_stdio(false);
  cin >> n >> k;
  int m = n / k;
  REP(i, 0, m) {
    cin >> a[i];
  }
  REP(i, 0, m) {
    cin >> b[i];
  }
  ll sum = 1;
  REP(i, 0, m) {
    sum *= f(a[i], b[i], k);
    sum %= mod;
  }
  cout << sum << "\n";
}
