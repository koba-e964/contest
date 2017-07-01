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


void fail(void) {
  cout << -1 << endl;
  exit(0);
}

int main(void){
  int n, m;
  cin >> n >> m;
  VI l(m);
  REP(i, 0, m) {
    cin >> l[i];
    l[i]--;
  }
  VI a(n, -1);
  REP(i, 0, m - 1) {
    int diff = (l[i + 1] - l[i] + n - 1) % n;
    int &ent = a[l[i]];
    if (ent == -1) {
      ent = diff;
    } else if (ent != diff) {
      fail();
    }
  }
  set<int> used;
  REP(i, 0, n) {
    if (a[i] == -1) {
      continue;
    }
    if (used.count(a[i])) {
      fail();
    }
    used.insert(a[i]);
  }
  int cnt = 0;
  REP(i, 0, n) {
    if (a[i] != -1) { continue; }
    while (used.count(cnt)) { cnt += 1; }
    a[i] = cnt;
    cnt += 1;
  }
  REP(i, 0, n) {
    cout << a[i] + 1 << (i == n - 1 ? "\n" : " ");
  }
}
