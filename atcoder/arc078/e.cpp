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

ll n;

string str(ll v) {
  stringstream ss;
  ss << v;
  return ss.str();
}

int ask(ll q) {
  if (n == 0) {
    // actual output
    cout << "? " << q << endl;
    char res;
    cin >> res;
    return res == 'Y';
  }
  return (q <= n) == (str(q) <= str(n));
}

void output(ll q) {
  if (n == 0) {
    cout << "! " << q << endl;
  } else {
    cout << n << " == " << q << endl;
  }
  exit(0);
}


int main(void) {
  n = 0;
#if 0
  cin >> n;
#endif
  if (ask(1e9)) {
    // 10...0
    ll cur = 1;
    while (true) {
      if (ask(cur + 1)) {
	output(cur);
      }
      cur *= 10;
    }
    assert (0);
  }
  // not 10...0
  ll cur = 1;
  while (true) {
    if (not ask(cur)) {
      break;
    }
    cur *= 10;
  }
  // cur / 10 < n < cur
  ll lo = cur / 10;
  ll hi = cur;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    if (ask(10 * mid)) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  output(hi);
}
