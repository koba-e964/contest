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
  int n;
  cin >> n;
  vector<PI> hamko(n);
  REP(i, 0, n) {
    string s;
    cin >> s;
    int mi = 0, delta = 0;
    REP(i, 0, s.length()) {
      delta += s[i] == '(' ? 1 : -1;
      mi = min(mi, delta);
    }
    hamko[i] = PI(delta, mi);
  }
  map<int, VI> chino;
  REP(i, 0, n) {
    chino[hamko[i].first].push_back(hamko[i].second);
  }
  ll tot = 0;
  for (auto &e: chino) {
    int key = e.first;
    if (key < 0) continue;
    VI &vi = e.second;
    sort(vi.begin(), vi.end());
    if (key == 0) {
      ll len = vi.end() - lower_bound(vi.begin(), vi.end(), 0);
      tot += len * len;
    } else {
      VI &opp = chino[-key];
      sort(opp.begin(), opp.end());
      for (int mi: vi) {
        if (mi < 0) continue;
        ll len = opp.end() - lower_bound(opp.begin(), opp.end(), -key);
        tot += len;
      }
    }
  }
  cout << tot << "\n";
}
