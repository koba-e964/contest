#include <cassert>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

const pair<ll, ll> inf = make_pair(1e16, -1);

pair<ll, ll> solve_unary(ll a, ll b, ll x) {
  if (a == 0) {
    return x % b == 0 ? pair<ll, ll>(1, x / b) : inf;
  }
  if (b == 0) {
    return x % a == 0 ? pair<ll, ll>(x / a, 1) : inf;
  }
  if (a >= x) {
    return inf;
  }
  ll u = 1;
  if ((x - a) % b != 0) {
    return inf;
  }
  ll v = (x - a) / b;
  return u >= 1 && v >= 1 ? pair<ll, ll>(u, v) : inf;
}
pair<ll, ll> solve(const vector<VL> &A, const VL &b) {
  int n = A.size();
  int m = A[0].size();
  assert (b.size() == n);
  assert (m == 2);
  assert (n >= 2);
  ll det = A[0][0] * A[1][1] - A[1][0] * A[0][1];
  ll u = (A[1][1] * b[0] - A[0][1] * b[1]) / det;
  ll v = (-A[1][0] * b[0] + A[0][0] * b[1]) / det;
  REP(i, 0, n) {
    if (A[i][0] * u + A[i][1] * v != b[i]) {
      return inf;
    }
  }
  return u >= 1 && v >= 1 ? pair<ll, ll>(u, v) : inf;
}

int main(void){
  VL x(3);
  cin >> x[0] >> x[1] >> x[2];
  int ma = max(x[0], max(x[1], x[2]));
  VL fib;
  {
    ll a = 1;
    ll b = 0;
    while (a <= ma) {
      fib.push_back(a);
      ll c = a + b;
      a = b;
      b = c;
    }
  }
  int m = fib.size();
  {
    set<ll> q;
    REP(i, 0, 3) { q.insert(x[i]); }
    x = VL(q.begin(), q.end());
  }
  pair<ll, ll> mi = inf;
  if (x.size() == 1) {
    REP(i, 0, m - 1) {
      pair<ll, ll> res = solve_unary(fib[i], fib[i + 1], x[0]);
      mi = min(mi, res);
    }
  }
  if (x.size() == 2) {
    REP(i, 0, m - 1) {
      REP(j, i + 1, m - 1) {
	vector<VL> A(2, VL(2));
	A[0][0] = fib[i]; A[0][1] = fib[i + 1];
	A[1][0] = fib[j]; A[1][1] = fib[j + 1];
	pair<ll, ll> res = solve(A, x);
	mi = min(mi, res);
      }
    }
  }
  if (x.size() == 3) {
    REP(i, 0, m - 1) {
      REP(j, i + 1, m - 1) {
	REP(k, j + 1, m - 1) {
	  vector<VL> A(3, VL(2));
	  A[0][0] = fib[i]; A[0][1] = fib[i + 1];
	  A[1][0] = fib[j]; A[1][1] = fib[j + 1];
	  A[2][0] = fib[k]; A[2][1] = fib[k + 1];
	  pair<ll, ll> res = solve(A, x);
	  mi = min(mi, res);
	}
      }
    }
  }
  if (mi.second == -1) {
    cout << -1 << endl;
  } else {
    cout << mi.first << " " << mi.second << endl;
  }
}
