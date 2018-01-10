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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int C = 30;
vector<PI> pool[C];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, cc;
  cin >> n >> cc;
  REP(i, 0, n) {
    int s, t, c;
    cin >> s >> t >> c;
    c--;
    pool[c].push_back(PI(s, t));
  }
  vector<PI> all;
  REP(i, 0, cc) {
    sort(pool[i].begin(), pool[i].end());
    vector<PI> tmp(pool[i]);
    pool[i].clear();
    int st = -1;
    int en = -1;
    REP(i, 0, tmp.size()) {
      if (tmp[i].first != en) {
	if (en >= 0) {
	  all.push_back(PI(2 * st - 1, 1));
	  all.push_back(PI(2 * en, -1));
	}
	st = tmp[i].first;
	en = tmp[i].second;
      } else {
	en = tmp[i].second;
      }
    }
    if (st >= 0) {
      all.push_back(PI(2 * st - 1, 1));
      all.push_back(PI(2 * en, -1));
    }
  }
  sort(all.begin(), all.end());
  int ma = 0;
  int cur = 0;
  REP(i, 0, all.size()) {
    cur += all[i].second;
    ma = max(ma, cur);
  }
  cout << ma << endl;
}
