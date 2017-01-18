#include <cassert>
#include <iostream>
#include <list>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

ll gcd(ll a, ll b) {
  return b == 0 ? a : gcd(b, a % b);
}

ll ex_gcd(ll x, ll y, ll &a, ll &b) {
  if (y == 0) {
    a = 1;
    b = 0;
    return x;
  }
  ll q = x / y;
  ll r = x % y;
  ll res = ex_gcd(y, r, a, b);
  ll tmp = a - q * b;
  a = b;
  b = tmp;
  return res;
}


/*
 * Calculates the intersection of two arithmetic progressions,
 * x[n] = a + b * n and y[n] = c + d * n (n >= 0).
 * p1 = (a, b), p2 = (c, d)
 * Verified by: yukicoder No.261 (http://yukicoder.me/submissions/144768)
 */
pair<ll, ll> arith_prog_intersect(const pair<ll, ll> &p1, const pair<ll, ll> &p2) {
  if (p1.first > p2.first) {
    return arith_prog_intersect(p2, p1);
  }
  ll u, v;
  ll g = ex_gcd(p1.second, p2.second, u, v);
  ll lcm = p1.second / g * p2.second;
  if ((p1.first - p2.first) % g != 0) {
    return pair<ll, ll>(-1, -1);
  }
  ll diff = (p2.first - p1.first) / g;
  diff *= -v % (p1.second / g);
  diff %= p1.second / g;
  if (diff < 0) {
    diff += p1.second / g;
  }
  ll x = p2.first + diff * p2.second;
  return pair<ll, ll>(x, lcm);
}

int main(void){
  int n, k;
  cin >> n >> k;
  int a[100] = {};
  int dp[100] = {};
  REP(i, 0, n) {
    a[i] = i;
    dp[i] = -1;
  }
  REP(i, 0, k) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    swap(a[x], a[y]);
  }
  REP(i, 0, n) {
    if (dp[i] > 0) {
      continue;
    }
    int c = 1;
    int v = a[i];
    while (v != i) {
      v = a[v];
      c++;
    }
    while (dp[v] < 0) {
      dp[v] = c;
      v = a[v];
    }
  }
  int q;
  cin >> q;
  REP(loop_cnt, 0, q) {
    VI b(n);
    REP(i, 0, n) {
      cin >> b[i];
      b[i]--;
    }
    bool ok = true;
    pair<ll, ll> coef(1, 1);
    REP(i, 0, n) {
      int c = 0;
      int v = i;
      int goal = b[i];
      while (c <= dp[i]) {
	if (goal == v) {
	  break;
	}
	v = a[v];
	c++;
      }
      if (c > dp[i]) {
	ok = false;
	break;
      }
      if (0) {
	cerr << "y(" << i << ")= " << c << " + " << dp[i] << " * x" << endl;
      }
      coef = arith_prog_intersect(coef, pair<ll, ll>(c, dp[i]));
      if (0) {
	cerr << "coef = (" << coef.first << "," << coef.second << ")" << endl;
      }
      if (coef.first < 0) {
	ok = false;
	break;
      }
    }
    if (not ok) {
      cout << -1 << endl;
      continue;
    }
    cout << coef.first << endl;
  }
  
}
