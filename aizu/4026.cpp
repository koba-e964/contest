#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  // oo, ox/xo, xx だけ試せば良い。他のパターンでできるならこれでもできる。
  int ma = 0;
  // oo, xx
  REP(iter, 0, 2) {
    char target = "ox"[iter];
    int bl = 0;
    int cnt = 0;
    for (auto c: s) {
      if (c == target) {
        cnt++;
      } else {
        cnt = 0;
      }
      if (cnt >= 2) {
        bl++;
        cnt = 0;
      }
    }
    ma = max(ma, bl);
  }
  // ox/xo
  {
    int bl = 0;
    int pos = 0;
    while (pos < (int)s.size()) {
      string cut = s.substr(pos, 2);
      if (cut == "ox" || cut == "xo") {
        bl++;
        pos += 2;
      } else {
        pos++;
      }
    }
    ma = max(ma, bl);
  }
  cout << ma << endl;
}
