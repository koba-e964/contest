#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll p, y;
  cin >> p >> y;
  for (ll i = y; i > p; --i) {
    bool div = false;
    for (ll j = 2; j <= p && j * j <= i; ++j) {
      if (i % j == 0) {
	div = true;
	break;
      }
    }
    if (not div) {
      cout << i << endl;
      return 0;
    }
  }
  cout << -1 << endl;
}
