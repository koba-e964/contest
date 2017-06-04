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

const int N = 393939;
VI edges[N];

bool check0(VI v1) {
  assert (v1.size() >= 2);
  set<int> qs;
  REP(i, 0, v1.size()) {
    int v = v1[i];
    VI nb(1, v);
    REP(j, 0, edges[v].size()) {
      nb.push_back(edges[v][j]);
    }
    if (i == 0) {
      REP(j, 0, nb.size()) {
	qs.insert(nb[j]);
      }
    } else { // retain
      set<int> qnew;
      REP(j, 0, nb.size()) {
	if (qs.count(nb[j])) {
	  qnew.insert(nb[j]);
	}
      }
      qs = qnew;
    }
    if (qs.size() == 0) {
      return false;
    }
  }
  return qs.size() != 0;
}


int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  ll ma = a[0];
  REP(i, 1, n) {
    ma = max(ma, a[i]);
  }
  // ma, ma + 1 or ma + 2 are possible
  REP(d, 0, 3) {
    ll x = ma + d;
    int num2 = 0;
    int v2 = -1;
    int num1 = 0;
    VI v1;
    REP(i, 0, n) {
      ll diff = x - 2 - a[i];
      assert (diff >= -2);
      if (diff == -2) {
	num2 += 1;
	v2 = i;
      } else if (diff == -1) {
	num1 += 1;
	v1.push_back(i);
      }
    }
    bool ok = true;
    if (num2 >= 2) {
      ok = false;
      continue;
    }
    if (num2 == 1) {
      int kuwa = 0;
      REP(i, 0, edges[v2].size()) {
	int w = edges[v2][i];
	if (x - 2 - a[w] == -1) {
	  kuwa += 1;
	}
      }
      // pigeonhole principle
      if (kuwa != num1) {
	ok = false;
        continue;
      }
    }
    if (num2 == 0) {
      if (num1 <= 1) {
	ok = true;
      } else {
	ok = check0(v1);
      }
    }
    if (ok) {
      cout << x << endl;
      return 0;
    }
  }
  assert (0);
}
