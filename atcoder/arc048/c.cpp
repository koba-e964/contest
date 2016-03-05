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

ll pow2(ll e) {
  ll sum = 1;
  ll cur = 2;
  while (e > 0) {
    if (e % 2 == 1) {
      sum *= cur;
      sum %= mod;
    }
    cur *= cur;
    cur %= mod;
    e /= 2;
  }
  return sum;
}

int main(void){
  int n;
  cin >> n;
  VI l(n);
  REP(i, 0, n) {
    cin >> l[i];
  }
  sort(l.begin(), l.end());
  int lm = 0x7fffffff;
  VI tt;
  REP(i, 0, n) {
    lm = min(lm, l[i]);
  }
  REP(i, 0, n - 1) {
    if (l[i] != l[i + 1]) {
      tt.push_back(abs(l[i] - l[i + 1]));
    }
  }
  int lg = tt.size() == 0 ? -1 : tt[0];
  REP(i, 1, tt.size()) {
    lg = __gcd(lg, tt[i]);
  }
  cout << pow2(lm + (lg + 1) / 2) << endl;
}
