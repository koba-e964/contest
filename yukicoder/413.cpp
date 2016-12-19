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

ll calc(ll d) {
  return (ll)((-1 + sqrt(1 + 4*d)) / 2.0);
}

const ll lim = 1e8;
int main(void){
  int pos = 0;
  ll i = 0;
  while (i < lim && pos < 100000) {
    ll d = lim - i - 1;
    d *= d + 1;
    REP(j, 0, 10) {
      ll t = calc(d);
      if (t * t + t <= d && (t + 1) * (t + 2) > d) {
      } else {
	cout << d << endl;
	pos++;
      }
      d--;
    }
    ++i;
  }
}
