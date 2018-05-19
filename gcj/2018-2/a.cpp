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

vector<string> solve(VI b) {
  int c = b.size();
  if (b[0] == 0 || b[c - 1] == 0) return vector<string>();
  VI to;
  REP(i, 0, c) {
    REP(j, 0, b[i]) to.push_back(i);
  }
  assert (to[0] == 0);
  assert (to[c - 1] == c - 1);
  int row = 0;
  REP(i, 0, c) {
    row = max(row, abs(to[i] - i));
  }
  row += 1;
  vector<string> ans(row, string(c, '.'));
  REP(i, 0, c) {
    if (to[i] < i) {
      REP(j, 0, i - to[i]) {
	ans[j][i - j] = '/';
      }
    }
    if (to[i] > i) {
      REP(j, 0, to[i] - i) {
	ans[j][i + j] = '\\';
      }
    }
  }
  return ans;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int c;
    cin >> c;
    VI b(c);
    REP(i, 0, c) cin >> b[i];
    vector<string> ans = solve(b);
    cout << "Case #" << case_nr << ": ";
    if (ans.size() == 0) {
      cout << "IMPOSSIBLE" << endl;
    } else {
      cout << ans.size() << endl;
      REP(i, 0, ans.size()) cout << ans[i] << endl;
    }
  }
}
