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


ll solve(const string &s) {
  vector<int> dig(s.length());
  int ma = 0;
  REP (i, 0, s.length()) {
    if ('0' <= s[i] && s[i] <= '9') {
      dig[i] = s[i] - '0';
    } else {
      dig[i] = s[i] - 'A' + 10;
    }
    ma = max(ma, dig[i]);
  }
  ma++;
  ll ans = 0;
  REP(i, 0, dig.size()) {
    ans *= ma;
    ans += dig[i];
  }
  return ans;
}

int main(void){
  int n;
  ll mi;
  string s;
  cin >> n;
  cin >> s;
  mi = solve(s);
  REP(i, 1, n) {
    cin >> s;
    mi = min(mi, solve(s));
  }
  cout << mi << endl;
}
