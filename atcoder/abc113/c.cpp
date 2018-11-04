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
  int n, m;
  cin >> n >> m;
  VI p(m), y(m);
  vector<vector<PI> > tr(n + 1);
  REP(i, 0, m) {
    cin >> p[i] >> y[i];
    tr[p[i]].push_back(PI(y[i], i));
  }
  vector<PI> ans(m);
  REP(i, 1, n + 1) {
    sort(tr[i].begin(), tr[i].end());
    REP(j, 0, tr[i].size()) {
      ans[tr[i][j].second] = PI(i, j + 1);
    }
  }
  REP(i, 0, m) {
    printf("%06d%06d\n", ans[i].first, ans[i].second);
  }
}
