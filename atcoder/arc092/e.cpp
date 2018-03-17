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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  pair<ll, int> ma(-1e18, -1);
  vector<VI> pop(2);
  REP(i, 0, 2) {
    ll tot = 0;
    pair<ll, int> ma2(-1e18, -1);
    REP(j, 0, n) {
      if (j % 2 != i) continue;
      if (a[j] > 0) {
	pop[i].push_back(j);
	tot += a[j];
      }
      ma2 = max(ma2, make_pair(a[j], j));
    }
    if (pop[i].size() == 0) {
      tot = ma2.first;
      pop[i].push_back(ma2.second);
    }
    ma = max(ma, make_pair(tot, i));
  }
  int ind = ma.second;
  VI arr = pop[ind];
  assert (arr.size() > 0);
  VI op;
  int fst = arr[0];
  int lst = arr[arr.size() - 1];
  for (int i = n - 1; i >= lst + 1; --i) {
    op.push_back(i);
  }
  REP(i, 0, fst) {
    op.push_back(0);
  }
  REP(i, 0, arr.size() - 1) {
    int intv = arr[i + 1] - arr[i];
    assert (intv % 2 == 0);
    for (int j = intv / 2; j >= 1; --j) {
      op.push_back(j);
    }
  }
  cout << ma.first << endl;
  cout << op.size() << endl;
  REP(i, 0, op.size()) {
    cout << op[i] + 1 << endl;
  }
}
