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

PI parse_num(string expr) {
  int cur = 0;
  int pos = 0;
  int n = (int) expr.length();
  while (pos < n) {
    if (not isdigit(expr[pos])) {
      return pos == 0 ? PI(-1, -1) : PI(pos, cur);
    }
    cur *= 10;
    cur += expr[pos] - '0';
    pos++;
  }
  return n == 0 ? PI(-1, -1) : PI(n, cur);
}

PI eval_decl(map<string, int> &ary, 
	     map<string, map<int, int> > &asgn,
	     string expr) {
  int n = expr.length();
  string an = expr.substr(0, 1);
  if (n <= 2 || expr[1] != '[') {
    return PI(-1, -1);
  }
  PI sz = parse_num(expr.substr(2));
  if (sz.first == -1) {
    return PI(-1, -1);
  }
  int len = sz.first + 2;
  if (n <= len || expr[len] != ']') {
    return PI(-1, -1);
  }
  // Should be exact
  if (n != len + 1) {
    return PI(-1, -1);
  }
  ary[an] = sz.second;
  return PI(len + 1, 0);
}

PI eval_expr(map<string, int> &ary, 
	     map<string, map<int, int> > &asgn,
	     string expr) {
  PI num = parse_num(expr);
  if (num.first >= 0) {
    return num;
  }
  // a[idx]
  string an = expr.substr(0, 1);
  int n = expr.length();
  if (n <= 2 || expr[1] != '[') {
    return PI(-1, -1);
  }
  PI idx = eval_expr(ary, asgn, expr.substr(2));
  if (idx.first == -1) {
    return PI(-1, -1);
  }
  int len = 2 + idx.first;
  if (n <= len || expr[len] != ']') {
    return PI(-1, -1);
  }
  if (ary.count(an) == 0) {
    return PI(-1, -1);
  }
  int arylen = ary[an];
  if (idx.second < 0 || idx.second >= arylen || asgn[an].count(idx.second) == 0) {
    return PI(-1, -1);
  }
  return PI(len + 1, asgn[an][idx.second]);
  
}

PI eval_asgn(map<string, int> &ary, 
	     map<string, map<int, int> > &asgn,
	     string expr) {
  string an = expr.substr(0, 1);
  int n = expr.length();
  if (n <= 2 || expr[1] != '[') {
    return PI(-1, -1);
  }
  PI idx = eval_expr(ary, asgn, expr.substr(2));
  if (idx.first == -1) {
    return PI(-1, -1);
  }
  int len = 2 + idx.first;
  if (n <= len + 1 || expr[len] != ']' || expr[len + 1] != '=') {
    return PI(-1, -1);
  }
  PI elem = eval_expr(ary, asgn, expr.substr(len + 2));
  if (elem.first == -1) {
    return PI(-1, -1);
  }
  if (ary.count(an) == 0) {
    return PI(-1, -1);
  }
  int arylen = ary[an];
  if (idx.second < 0 || idx.second >= arylen) {
    return PI(-1, -1);
  }
  asgn[an][idx.second] = elem.second;
  return PI(len + 2 + elem.first, 0);
}

PI eval(map<string, int> &ary, 
	 map<string, map<int, int> > &asgn,
	 string expr) {
  PI res = eval_decl(ary, asgn, expr);
  if (res.first >= 0) {
    return res;
  }
  return eval_asgn(ary, asgn, expr);
}


void solve(const vector<string> &pool) {
  map<string, int> ary;
  map<string, map<int, int> > asgn;
  REP(i, 0, pool.size()) {
    PI res = eval(ary, asgn, pool[i]);
    if (res.first == -1) {
      cout << i + 1 << "\n";
      return;
    }
  }
  cout << 0 << "\n";
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  vector<string> pool;
  string cur;
  while(cin >> cur) {
    if (cur == ".") {
      if (pool.empty()) {
	return 0;
      }
      solve(pool);
      pool.clear();
    } else {
      pool.push_back(cur);
    }
  }
}
