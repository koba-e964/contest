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

int solve(string &s, int v) {
  VI ans(10);
  int tot = 0;
  REP(i, 0, 10) {
    s[v] = '0' + i;
    cout << s << endl;
    int ret; string lock;
    cin >> ret >> lock;
    if (lock == "unlocked") {
      return 1;
    }
    ans[i] = ret;
    tot += ret;
  }
  REP(i, 0, 10) {
    if (ans[i] == tot / 10 + 1) {
      s[v] = '0' + i;
      return 0;
    }
  }
  assert(0);
}

int main(void){
  string s = "0000000000";
  REP(i, 0, 10) {
    if (solve(s, i)) {
      return 0;
    }
  }
}
