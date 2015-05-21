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

const ll err = 1LL << 60;

typedef pair<int, ll> result;

result p_int(string s, int v) {
  if (v >= s.length()) {
    return result(0, err);
  }
  ll q = 0;
  int len = 0;
  for (; v + len < s.length(); ++len) {
    char c = s[v + len];
    if (('0' <= c && c <= '9') == false) {
      break;
    }
    q *= 10;
    q += c - '0';
  }
  if (len >= 1) {
    return result(len, q);
  }
  return result(0, err);
}


result p_expr(const string &s, int v) {
  ll res;
  int pos = v;
  result r1 = p_int(s, v);
  if (r1.first == 0) {
    return result(0, err);
  }
  pos += r1.first;
  res = r1.second;
  while (pos < s.length()) {
    char op = s[pos];
    switch (op) {
    case '+':
    case '-':
      break;
    default:
      return result(pos - v, res);
    }
    result sub = p_int(s, pos + 1);
    if (sub.first == 0) {
      return result(pos - v, res);
    }
    res = op == '+' ? res + sub.second : res - sub.second;
    pos += sub.first + 1;
  }
  return result(s.length() - v, res);
}



int main(void){
  string s;
  cin >> s;
  int n = s.length();
  ll m = -234567892345LL;
  REP(i, 0, n) {
    string t = s.substr(i, n) + s.substr(0, i);
    result r = p_expr(t, 0);
    if (r.first != n) {
      continue;
    }
    m = max(m, r.second);
  }
  cout << m << endl;
}
