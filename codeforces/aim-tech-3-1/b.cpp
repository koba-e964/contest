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

int check(ll v) {
  int len = -1;
  REP(i, 0, 1000001) {
    v -= i;
    if (v == 0) {
      len = i + 1;
    }
  }
  return len;
}

int main(void){
  ll a, b, c, d;
  cin >> a >> b >> c >> d;
  ll k = a + b + c + d;
  int len = check(k);
  int n0 = check(a);
  int n1 = check(d);
  if (len == 1) {
    // a = b = c = d = 0
    cout << 0 << endl;
    return 0;
  }
  if (len == -1 || n0 == -1 || n1 == -1) {
    cout << "Impossible" << endl;
    return 0;
  }
  assert (len >= 2);
  if (n0 == 1 && len == n1) {
    n0 = 0;
  }
  if (n1 == 1 && len == n0) {
    n1 = 0;
  }
  if (len != n0 + n1) {
    cout << "Impossible" << endl;
    return 0;
  }
  // make up such a string
  string s;
  s.reserve(len + 1);
  if (n1 == 0) {
    REP(i, 0, len) {
      s += '0';
    }
    cout << s << endl;
    return 0;
  }
  int q = c / n1;
  int r = c % n1;
  if (r >= 1) {
    REP(i, 0, n0 - q - 1) {
      s += '0';
    }
    REP(i, 0, r) {
      s += '1';
    }
    s += '0';
    REP(i, 0, n1 - r) {
      s += '1';
    }
    REP(i, 0, q) {
      s += '0';
    }
  } else {
    REP(i, 0, n0 - q) {
      s += '0';
    }
    REP(i, 0, n1) {
      s += '1';
    }
    REP(i, 0, q) {
      s += '0';
    }
  }
  cout << s << endl;
}
