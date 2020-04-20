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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int r, s;
    cin >> r >> s;
    VI arr(r * s);
    REP(i, 0, s) {
      REP(j, 0, r) {
        arr[i * r + j] = j;
      }
    }
    cout << "Case #" << case_nr << ": ";
    vector<PI> ans;
    if (r % 2 == 0) {
      REP(i, 0, s - 1) {
        REP(j, 0, r / 2) {
          int to = (s - i - 1) * r + (2 * j + 1) * (i + 1);
          ans.push_back(PI(2, to - 2));
        }
      }
    } else {
      VI pos(r);
      REP(i, 0, r) pos[i] = (s - 1) * r + i;
      int k = ((s - 1) * r - 1) / 2;
      REP(i, 0, k) {
        int b = (2 * i + 1) % r;
        int to;
        if (b == 0) {
          to = pos[0];
        } else {
          to = pos[b];
        }
        ans.push_back(PI(2, to - 2));
        REP(i, 0, b) {
          pos[i] -= 2;
        }
        pos[b] -= 1;
        if (DEBUG) {
          DEBUGP(b);
          REP(i, 0, r) cerr << " " << pos[i];
          cerr << endl;
        }
      }
      ans.push_back(PI(pos[0], pos[r - 1] - pos[0]));
    }
    cout << ans.size() << "\n";
    REP(i, 0, ans.size()) {
      int x = ans[i].first, y = ans[i].second;
      cout << x << " " << y << "\n";
      rotate(arr.begin(), arr.begin() + x, arr.begin() + x + y);
    }
    if (DEBUG) {
      REP(i, 0, r * s) cerr << " " << arr[i];
      cerr << endl;
    }
  }
}
