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
  cout << "NO\n";
  exit(0);
}

int check(vector<string> &s, int x1, int x2, int y1, int y2) {
  set<char> used;
  REP(i, x1, x2) {
    REP(j, y1, y2) {
      used.insert(s[i][j]);
    }
  }
  if (used.size() >= 2) {
    return 0;
  }
  if (used.count('R')) {
    return 1;
  }
  if (used.count('B')) {
    return 2;
  }
  if (used.count('G')) {
    return 3;
  }
  return -1;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  if (n * m % 3 != 0) {
    fail();
  }
  bool ok = false;
  if (n % 3 == 0) {
    set<int> cols;
    cols.insert(check(s, 0, n / 3, 0, m));
    cols.insert(check(s, n / 3, 2 * n / 3, 0, m));
    cols.insert(check(s, 2 * n / 3, n, 0, m));
    if (cols.count(0) == 0 && cols.size() == 3) {
      ok = true;
    }
  }
  if (m % 3 == 0) {
    set<int> cols;
    cols.insert(check(s, 0, n, 0, m / 3));
    cols.insert(check(s, 0, n, m / 3, 2 * m / 3));
    cols.insert(check(s, 0, n, 2 * m / 3, m));
    if (cols.count(0) == 0 && cols.size() == 3) {
      ok = true;
    }
  }
  cout << (ok ? "YES" : "NO") << "\n";
}
