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
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    ll x, y;
    cin >> x >> y;
    cout << "Case #" << case_nr << ": ";
    string s;
    bool ok = 1;
    while (ok && (x != 0 || y != 0)) {
      if (((x ^ y) & 1) == 0) {
        ok = 0;
        break;
      }
      if (y == 0 && abs(x) == 1) {
        s += x == 1 ? "E" : "W";
        break;
      }
      if (x == 0 && abs(y) == 1) {
        s += y == 1 ? "N" : "S";
        break;
      }
      if (x & 1) {
        if ((x ^ y) & 2) {
          s += "E";
          x -= 1;
        } else {
          s += "W";
          x += 1;
        }
      } else {
        if ((x ^ y) & 2) {
          s += "N";
          y -= 1;
        } else {
          s += "S";
          y += 1;
        }
      }
      x /= 2;
      y /= 2;
    }
    if (ok) {
      cout << s;
    } else {
      cout << "IMPOSSIBLE";
    }
    cout << "\n";
  }
}
