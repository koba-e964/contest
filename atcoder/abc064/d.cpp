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

int trans(string s, char c) {
  int cur = 0;
  int hi = 0;
  REP(i, 0, s.length()) {
    int tmp = s[i] == c ? 1 : -1;
    cur += tmp;
    hi = max(hi, cur);
  }
  return hi;
}

int main(void){
  int n;
  cin >> n;
  string s;
  cin >> s;
  int hi = trans(s, ')');
  REP(i, 0, hi) {
    s = "(" + s;
  }
  int lo = 0;
  REP(i, 0, s.length()) {
    int tmp = s[i] == '(' ? 1 : -1;
    lo += tmp;
  }
  REP(i, 0, lo) {
    s = s + ")";
  }
  cout << s << endl;
}
