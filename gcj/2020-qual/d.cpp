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

int ask(int idx) {
  cout << idx + 1 << endl;
  int a;
  cin >> a;
  return a;
}

int main(void) {
  int t, b;
  cin >> t >> b;
  REP(_, 0, t) {
    VI ans(b);
    int rnd = (b - 10 + 7) / 8;
    int eq = -1, ne = -1;
    REP(i, 0, 5) {
      ans[i] = ask(i);
      ans[b - 1 - i] = ask(b - 1 - i);
      if (ans[i] == ans[b - 1 - i]) {
        eq = i;
      } else {
        ne = i;
      }
    }
    REP(i, 0, rnd) {
      // 2 + 8
      int rem = 2;
      if (eq >= 0) {
        int a1 = ask(eq);
        rem--;
        if (a1 != ans[eq]) {
          // complement
          REP(j, 0, b) ans[j] = !ans[j];
        }
      }
      if (ne >= 0) {
        int a1 = ask(ne);
        rem--;
        if (a1 != ans[ne]) {
          // reverse
          reverse(ans.begin(), ans.end());
        }
      }
      REP(_, 0, rem) ask(0);
      int st = 5 + 4 * i;
      REP(j, st, st + 4) {
        ans[j] = ask(j);
        ans[b - 1 - j] = ask(b - 1 - j);
        if (ans[j] == ans[b - 1 - j]) {
          eq = j;
        } else {
          ne = j;
        }
      }
    }
    REP(i, 0, b) cout << ans[i];
    cout << endl;
    string v;
    cin >> v;
    if (v == "N") {
      return 1;
    }
  }
}
