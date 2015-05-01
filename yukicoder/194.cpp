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

const long long int mod = 1e9 + 7;

/* Requirement: vector
   T mod
*/
template<class T>
class Matrix {
private:
  int n;
  std::vector<std::vector<T> > mat;
public:
  Matrix(int n) : n(n), mat(n) {
    for (int i = 0; i < n; ++i) {
      mat[i] = std::vector<T>(n);
    }
  }
  std::vector<T> &operator[] (int i) {
    return mat[i];
  }
  const std::vector<T> &operator[] (int i) const {
    return mat[i];
  }
  Matrix<T> operator *(Matrix<T> const& ano) const {
    Matrix<T> ret(n);
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
	for (int k = 0; k < n; ++k) {
	  ret[i][j] += (*this)[i][k] * ano[k][j];
	  ret[i][j] %= mod;
	}
      }
    }
    return ret;
  }
  static Matrix<T> unit(int n) {
    Matrix<T> ret(n);
    for (int i = 0; i < n; ++i) {
      ret[i][i] = 1;
    }
    return ret;
  }
  template<class Integral>
  Matrix<T> pow(Integral exp) const {
    Matrix<T> sum = unit(n);
    Matrix<T> cur = *this;
    while (exp > 0) {
      if (exp % 2 != 0) {
	sum = sum * cur;
      }
      cur = cur * cur;
      exp /= 2;
    }
    return sum;
  }
};

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;


const int N = 10010;
int a[N];
int acc[N];
int n;
ll k;

/* k is small */
void small() {
  a[n] = 0;
  ll sum = 0;
  REP(i, 0, n) {
    a[n] += a[i];
  }
  sum = a[n] * 2 % mod;
  REP(i, n + 1, k) {
    int c = i % (n + 1);
    ll s = 2 * a[(i + n) % (n + 1)];
    s += mod - a[c];
    a[c] = s % mod;
    sum += a[c];
    sum %= mod;
  }
  cout << a[(k - 1) % (n + 1)] << " " << sum << endl;
}


void large() {
  Matrix<ll> cp(n);
  REP(i, 0, n - 1) {
    cp[i][i + 1] = 1;
  }
  REP(i, 0, n) {
    cp[n - 1][i] = 1;
  }
  cp = cp.pow(k - 1);
  ll val = 0;
  REP(i, 0, n) {
    val += cp[0][i] * a[i];
    val %= mod;
  }
  Matrix<ll> sm(n + 1);
  REP(i, 0, n) {
    sm[i][i + 1] = 1;
  }
  sm[n][0] = mod - 1;
  sm[n][n] = 2;
  sm = sm.pow(k - 1);
  ll sum = 0;
  acc[0] = a[0];
  REP(i, 1, n) {
    acc[i] = acc[i - 1] + a[i];
    acc[i] %= mod;
  }
  acc[n] = 2 * acc[n - 1] % mod;
  REP(i, 0, n + 1) {
    sum += sm[0][i] * acc[i];
    sum %= mod;
  }
  
  cout << val << " " << sum << endl;
}

int main(void){
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> a[i];
  }
  if (n >= 31) {
    small();
    return 0;
  }
  large();
}
