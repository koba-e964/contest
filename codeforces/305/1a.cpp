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


ll m;
ll h1, a1,xx1,yy1,h2,a2,xx2,yy2;
const int N = 2e6+1;
ll uu1[N], uu2[N];
const int DEBUG = 0;
const ll inf = 123456789LL << 32;

ll solve(ll a1, ll a2, ll m1, ll m2) {
  if (a1 == -1 || a2 == -1) {
    return inf;
  }
  ll g = __gcd(m1, m2);
  ll lcm = m1 / g * m2;
  if (a1 % g != a2 % g) {
    return inf;
  }
  REP (i, 0, m2 / g) {
    ll res = a1 + m1 * i;
    if (res % m2 == a2 % m2) {
      res %= lcm;
      if (DEBUG) {
	cout <<"solve(" << a1 <<"," << a2 << "," << m1 << "," << m2 << ")=" << res << endl;
      }
      return res;
    }
  }
  return inf;
}

int main(void){
  cin >> m;
  cin >> h1 >> a1 >> xx1 >> yy1;
  cin >> h2 >> a2 >> xx2 >> yy2;
  REP(i, 0, N) {
    uu1[i] = uu2[i] = -1;
  }
  ll p1 = -1, p2 = -1, s1 = -1, s2 = -1;
  REP (i, 0, 2 * m) {
    if (p1 == -1 && uu1[h1] != -1) {
      p1 = i - uu1[h1];
      s1 = uu1[h1];
    }
    if (p2 == -1 && uu2[h2] != -1) {
      p2 = i - uu2[h2];
      s2 = uu2[h2];
    }
    if (uu1[h1] == -1) {
      uu1[h1] = i;
    }
    if (uu2[h2] == -1) {
      uu2[h2] = i;
    }
    h1 = (h1 * xx1 + yy1) % m;
    h2 = (h2 * xx2 + yy2) % m;
  }
  if (DEBUG) {
    cout << "s1=" << s1 <<endl;
    cout << "s2=" << s2 <<endl;
    cout << "p1=" << p1 <<endl;
    cout << "p2=" << p2 <<endl;
  }
  ll b1 = uu1[a1];
  ll b2 = uu2[a2];
  if (b1 == -1 || b2 == -1) {
    cout << -1 << endl;
    return 0;
  }
  if (b1 < s1) {
    if (b2 < s2) {
      cout << (b1 == b2 ? b1 : -1) << endl;
    } else {
      cout << ((b1 >= b2 && (b1 - b2) % p2 == 0) ? b1 : -1) << endl;
    }
    return 0;
  }
  if (b2 < s2) {
    cout << ((b2 >= b1 && (b2 - b1) % p1 == 0) ? b2 : -1) << endl;
    return 0;
  }
  REP (i, 0, p2 + b2) {
    ll x = b1 + p1 * (ll)i;
    if (x >= b2 && (x - b2) % p2 == 0) {
      cout << x << endl;
      return 0;
    }
  }
  
  cout << -1 << endl;
}
