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
typedef pair<PI, int> PPII;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    cout << "Case #" << case_nr << ": ";
    int n, k;
    cin >> n >> k;
    int x = min(n - 1, k - 1);
    int sol = x * (2 * k - x - 1) / 2 - k;
    sol = max(sol, 0);
    cout << sol << endl;
    vector<PPII> edges;
    if (sol > 0) {
      REP(i, 0, x) {
	edges.push_back(PPII(PI(i, i == x - 1 ? n - 1 : i + 1), k - 1 - i));
      }
      edges.push_back(PPII(PI(0, n - 1), k));
    } else {
      edges.push_back(PPII(PI(0, n - 1), 1));
    }
    cout << edges.size() << endl;
    REP(i, 0, edges.size()) {
      PPII t = edges[i];
      PI td = t.first;
      assert (1 <= t.second);
      assert (t.second <= k);
      assert (td.first < n);
      assert (td.second < n);
      cout << td.first + 1 << " " << td.second + 1 << " " << t.second << endl;
    }
  }
}
