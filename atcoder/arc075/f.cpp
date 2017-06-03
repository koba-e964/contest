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

ll p10[19];

ll check(ll d, int k, int i) {
  if (2 * i >= k) {
    return d == 0 ? 1 : 0;
  }

  ll prod = 0;
  ll tmp = (p10[k - i] - p10[i]) / 9;
  int rem = (d / p10[i]) % 10;
  if (rem < 0) {
    rem += 10;
    rem %= 10;
  }
  REP(u, 0, 2) {
    ll uv = i == 0 ? 9 - abs(rem) : 10 - abs(rem);
    uv *= check(d - rem * tmp, k, i + 1);
    prod += uv;
    rem -= 10;
  }
  return prod;
}


ll calc(ll d, int k) {
  ll prod = 1;
  bool odd = k % 2 == 1;
  prod = check(d, k, 0);
  if (not odd) {
    prod *= 10;
  }
  return prod;
}

int main(void){
  ll d;
  cin >> d;
  if (d % 9 != 0) {
    cout << 0 << endl;
    return 0;
  }
  p10[0] = 1;
  REP(i, 0, 18) {
    p10[i + 1] = p10[i] * 10;
  }
  d /= 9;
  ll tot = 0;
  REP(k, 1, 19) {
    tot += calc(d, k);
  }
  cout << tot << endl;
}
