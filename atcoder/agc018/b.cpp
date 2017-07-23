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


// This code was written after I read the editorial
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<VI> a(n, VI(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      cin >> a[i][j];
      a[i][j]--;
    }
  }
  set<int> kind;
  REP(i, 0, m) {
    kind.insert(i);
  }
  int mi = 1e8;
  REP(loop_cnt, 0, m) {
    PI ma(-1, -1);
    vector<int> cnt(m);
    REP(i, 0, n) {
      int k = -1;
      REP(j, 0, m) {
	if (kind.count(a[i][j])) {
	  k = a[i][j];
	  break;
	}
      }
      cnt[k] += 1;
    }
    REP(i, 0, m) {
      ma = max(ma, PI(cnt[i], i));
    }
    mi = min(mi, ma.first);
    int v = ma.second;
    // remove v
    kind.erase(v);
  }
  cout << mi << "\n";
}
