#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

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
  double k;
  cin >> n >> k;
  k = log(k);
  vector<double> s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  REP(i, 0, n) {
    if (s[i] == 0) {
      cout << n << endl;
      return 0;
    }
  }
  vector<double> acc(n + 1, 0.0);
  REP(i, 0, n) {
    acc[i + 1] = acc[i] + log(s[i]);
  }
  vector<pair<double, int> > tbl(n + 1);
  REP(i, 0, n + 1) {
    tbl[i] = make_pair(acc[i], i);
  }
  sort(tbl.begin(), tbl.end());
  REP(i, 1, n + 1) {
    tbl[i].second = max(tbl[i].second, tbl[i - 1].second);
  }
  int ma = 0;
  REP(i, 0, n) {
    double to = acc[i] + k;
    int idx = upper_bound(tbl.begin(), tbl.end(), make_pair(to + 1e-13, -1))
      - tbl.begin() - 1;
    if (idx >= 0) {
      ma = max(ma, tbl[idx].second - i);
    }
  }
  cout << ma << "\n";
}
