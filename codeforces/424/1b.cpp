#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
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
  cin >> n;
  VI a(n);
  map<int, VI> pool;
  REP(i, 0, n) {
    cin >> a[i];
    pool[a[i]].push_back(i);
  }
  ll tot = 0;
  // Count #increasing passes
  int prev = -1;
  ll turn = n;
  int cc = 0;
  for (map<int, VI>::iterator it = pool.begin();
       it != pool.end(); ++it) {
    VI list = it->second;
    // sort (beginning: prev)
    vector<PI> beata;
    REP(j, 0, list.size()) {
      int idx = prev >= 0 ? (list[j] - prev + n) % n : list[j];
      beata.push_back(PI(idx, list[j]));
    }
    sort(beata.begin(), beata.end());
    REP(j, 0, beata.size()) {
      int p = beata[j].second;
      if (prev >= p) {
	tot += turn;
	turn = n - cc;
      }
      prev = p;
      cc += 1;
    }
  }
  tot += turn;
  cout << tot << "\n";
}
