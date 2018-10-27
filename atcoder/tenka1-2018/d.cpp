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
  int p = 0;
  while (1) {
    if (p * (p - 1) == 2 * n) {
      break;
    }
    if (p * (p - 1) > 2 * n) {
      cout << "No" << endl;
      return 0;
    }
    p++;
  }
  vector<VI> ans(p);
  int cur = 1;
  REP(i, 0, p) {
    REP(j, 0, i) {
      ans[i].push_back(cur);
      ans[j].push_back(cur);
      cur++;
    }
  }
  cout << "Yes" << endl;
  cout << p << endl;
  REP(i, 0, p) {
    cout << ans[i].size();
    REP(j, 0, ans[i].size()) {
      cout << " " << ans[i][j];
    }
    cout << endl;
  }
}
