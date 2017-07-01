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

string increment(string s) {
  int carry = 1;
  int n = s.length();
  for (int i = n - 1; i >= 0; --i) {
    s[i] += carry;
    carry = 0;
    if (s[i] > '9') {
      s[i] = '0';
      carry = 1;
    }
  }
  if (carry) {
    return '1' + s;
  }
  return s;
}


string calc(const string &s) {
  int start = -1;
  int end = -1;
  int n = s.length();
  bool prev = false;
  REP(i, 0, n) {
    if (isdigit(s[i])) {
      if (not prev) {
	start = i;
      }
      prev = true;
      end = i + 1;
    } else {
      prev = false;
    }
  }
  if (start < 0) {
    return s;
  }
  // increment [start, end)
  return s.substr(0, start) + increment(s.substr(start, end - start))
    + s.substr(end);
}

int main(void){
  int t;
  string first;
  getline(cin, first);
  stringstream ss(first);
  ss >> t;
  REP(i, 0, t) {
    string s;
    getline(cin, s);
    cout << calc(s) << endl;
  }
}
