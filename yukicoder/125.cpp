#include <algorithm>
#include <vector>
#include <cmath>
/*
 * Operations of power.
 * Header requirement: algorithm, vector, cmath
 */
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
  static std::pair<long long, int> toPower(long long n) {
    for (int i = 64; i >= 2; i--) {
      i64 app = std::pow(n, 1.0/i);
      for (int d = -4; d <= 4; d++) {
	i64 x = app + d;
	if (x <= 0) continue;
	if (power(x, i) == n) {
	  return std::pair<i64, int>(x, i);
	}
      }
    }
    return std::pair<i64, int>(n, 1);
  }
  /* factorize n and returns prime factors and their exponents.  O(sqrt(n)) */
  static std::vector<std::pair<long long, int> > factorize(long long n) {
    std::vector<std::pair<i64, int> > res;
    i64 p = 2;
    int c = 0;
    while (n >= 1) {
      if (c == 0 && n < p * p) {
	if(n != 1) {
	  res.push_back(std::pair<i64, int>(n,1));
	}
	break;
      }
      if (n % p != 0) {
	if (c > 0) {
	  res.push_back(std::pair<i64, int>(p,c));
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
  /*
   * Euler's totient function. O(sqrt(n))
   */
  static long long totient(long long n) {
    std::vector<std::pair<i64, int> > res = factorize(n);
    for (int i = 0; i < res.size(); ++i) {
      i64 p = res[i].first;
      n /= p;
      n *= p - 1;
    }
    return n;
  }
};
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const ll mod = 1e9 + 7;

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int F = 1e6 + 10;
ll ftbl[F];


const int K = 100100;
int c[K];

int main(void){
  ll tmp = 1;
  REP(i, 0, F) {
    ftbl[i] = tmp;
    tmp *= i + 1;
    tmp %= mod;
  }
  int k;
  cin >> k;
  int g = 0;
  int tot = 0;
  REP(i, 0, k) {
    cin >> c[i];
    g = __gcd(g, c[i]);
    tot += c[i];
  }
  ll cnt = 0;
  REP(f, 1, g + 1) {
    if (g % f != 0) {
      continue;
    }
    ll sum = ftbl[tot / f];
    REP(i, 0, k) {
      sum *= invmod(ftbl[c[i] / f]);
      sum %= mod;
    }
    sum *= Power::totient(f);
    cnt += sum;
    cnt %= mod;
  }
  cnt *= invmod(tot);
  cnt %= mod;
  cout << cnt << endl;
}
