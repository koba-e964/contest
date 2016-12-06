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


int intersect(VI &a, VI &b) {
  multiset<int> sa(a.begin(), a.end());
  int tot = 0;
  REP(i, 0, b.size()) {
    tot += sa.count(b[i]);
  }
  return tot;
}

int main(void){
  int n, m;
  cin >> n >> m;
  vector<VI> c(n * m);
  REP(i, 0, n * m) {
    c[i] = VI(n);
    REP(j, 0, n) {
      cin >> c[i][j];
    }
  }
  vector<vector<VI> > goal(m);
  REP(i, 0, m) {
    REP(j, 0, n) {
      VI s(n);
      REP(k, 0, n) { s[k] = c[i * n + j][k]; }
      goal[i].push_back(s);
      REP(k, 0, n) { s[k] = c[i * n + k][j]; }
      goal[i].push_back(s);
    }
    VI t(n);
    REP(j, 0, n) {
      t[j] = c[i * n + j][j];
    }
    goal[i].push_back(t);
    REP(j, 0, n) {
      t[j] = c[i * n + j][n - 1 - j];
    }
    goal[i].push_back(t);
    assert (goal[i].size() == 2 * n + 2);
  }
  int mi = 2 * n;
  REP(i, 0, m) {
    REP(j, i + 1, m) {
      REP(k, 0, 2 * n + 2) {
	REP(l, 0, 2 * n + 2) {
	  mi = min(mi, 2 * n - intersect(goal[i][k], goal[j][l]));
	}
      }
    }
  }
  cout << mi - 1 << endl;
}
