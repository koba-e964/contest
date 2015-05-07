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

const long long mod = 1e9 + 7;

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

pair<int, int> conv(const string &s, int mod) {
  ll a = 0;
  int over = 0;
  REP(i, 0, s.size()) {
    a *= 10;
    a += s[i] - '0';
    if (a >= mod) {
      over = 1;
    }
    a %= mod;
  }
  return pair<int, int>(a, over);
}


int main(void){
  int n;
  cin >> n;
  ll acc = 1;
  REP(i, 0, n) {
    ll c;
    string s;
    cin >> c >> s;
    PI dp = conv(s, mod - 1);
    int d = dp.second && dp.first == 0 ? mod - 1 : dp.first;
    Matrix<ll> m(2);
    m[0][0] = 0;
    m[0][1] = 1;
    m[1][0] = 1;
    m[1][1] = 1;
    m = m.pow(c);
    ll fc = m[0][0] + m[0][1] * 2;
    fc %= mod;
    ll cur = fc, sum = 1;
    while (d > 0) {
      if (d % 2) {
	sum *= cur;
	sum %= mod;
      }
      cur *= cur;
      cur %= mod;
      d /= 2;
    }
    acc *= sum;
    acc %= mod;
  }
  cout << acc << endl;
}
