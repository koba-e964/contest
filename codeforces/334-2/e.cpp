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

ll even_g(ll v) {
  if (v <= 2) {
    return v;
  }
  return (v - 1) % 2;
}

ll odd_g(ll v) {
  if (v <= 3) {
    return v % 2;
  }
  if (v == 4) {
    return 2;
  }
  if (v % 2 == 1) {
    return 0;
  }
  ll t = odd_g(v /2);
  return t == 1 ? 2 : 1;
}

int main(void){
  int n;
  ll k;
  cin >> n >> k;
  vector<ll> a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  if (k % 2 == 0) {
    ll tot = 0;
    REP(i, 0, n) {
      tot ^= even_g(a[i]);
    }
    cout << (tot ? "Kevin" : "Nicky") << endl;
    return 0;
  }

  ll tot = 0;
  REP(i, 0, n) {
    tot ^= odd_g(a[i]);
  }
  cout << (tot ? "Kevin" : "Nicky") << endl;

}
