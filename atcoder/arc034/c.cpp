#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>
 
using namespace std;
 
typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define rep(i,s,n) for(int i=(s); i < int(n); ++i)
 
class Power {
  typedef long long i64;
public:
  /* a^b with no modulo operations */
  static long long power(long long a, long long b) {
    i64 s = 1;
    i64 c = a;
    while (b > 0) {
      if (b % 2) {
	s *= c;
      }
      c *= c;
      b /= 2;
    }
    return s;
  }
  /* return (a,b) s.t, n = a^b and b is maximal. O(64) */
  static pair<long long, int> toPower(long long n) {
    for (int i = 64; i >= 2; i--) {
      i64 app = pow(n, 1.0/i);
      for (int d = -4; d <= 4; d++) {
	i64 x = app + d;
	if (x <= 0) continue;
	if (power(x, i) == n) {
	  return pair<i64, int>(x, i);
	}
      }
    }
    return pair<i64, int>(n, 1);
  }
  /* factorize n and returns prime factors and their exponents.  O(sqrt(n)) */
  static vector<pair<long long, int> > factorize(long long n) {
    vector<pair<i64, int> > res;
    i64 p = 2;
    int c = 0;
    while (n >= 1) {
      if (c == 0 && n < p * p) {
	if(n != 1) {
	  res.push_back(pair<i64, int>(n,1));
	}
	break;
      }
      if (n % p != 0) {
	if (c > 0) {
	  res.push_back(pair<i64, int>(p,c));
	}
	p++;
	c = 0;
	continue;
      }
      n /= p;
      c++;
    }
    return res;
  }
};
 
const int N = 1e6;
int a[N];
int n;
 
ll mod = 1e9 + 7;
int main(void) {
  int a, b;
  cin >> a >> b;
  if (a < b) {
    cout << 0 << endl;
    return 0;
  }
  vector<int> acc;
  rep(i, b + 1, a + 1) {
    vector<pair<ll, int> > res = Power::factorize(i);
    rep(j, 0, res.size()) {
      rep(k, 0, res[j].second) {
	acc.push_back(res[j].first);
      }
    }
  }
  sort(acc.begin(), acc.end());
  if(0) {
    cout << "acc:";
    rep(i,0,acc.size()) {
      cout << " " << acc[i];
    }
    cout << endl;
  }
  ll ans = 1;
  int cnt = 1;
  int cur = 0;
  rep(i, 0, acc.size()) {
    if (cur != acc[i]) {
      ans *= cnt;
      ans %= mod;
      cur = acc[i];
      cnt = 2;
    } else {
      cnt ++;
    }
  }
  ans *= cnt;
  ans %= mod;
  cout << ans << endl;
}
