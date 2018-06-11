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

// a connected components, with its complements connected
vector<VI> sigma(int n, int a) {
  assert (a >= 2);
  vector<VI> ret(n, VI(n, 0));
  REP(i, 0, n - a + 1) {
    REP(j, 0, n - a + 1) {
      if (i != j) ret[i][j] = 1;
    }
  }
  return ret;
}

void output(const vector<VI> &res) {
  cout << "YES\n";
  int n = res.size();
  REP(i, 0, n) {
    REP(j, 0, n) cout << res[i][j];
    cout << "\n";
  }
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, a, b;
  cin >> n >> a >> b;
  if (n == 1) {
    vector<VI> res(1, VI(1, 0));
    output(res);
  }
  if (a > 1 && b > 1) {
    cout << "NO\n";
    return 0;
  }
  if (a == 1 && b == 1) {
    // TODO
    if (n <= 3) {
      cout << "NO\n";
      return 0;
    }
    vector<VI> res(n, VI(n, 0));
    REP(i, 0, n - 1) {
      res[i][i + 1] = 1;
      res[i + 1][i] = 1;
    }
    output(res);
  }
  vector<VI> res = sigma(n, a == 1 ? b : a);
  if (a == 1) {
    REP(i, 0, n) {
      REP(j, 0, n) {
        if (i != j) res[i][j] = 1 - res[i][j];
      }
    }
  }
  output(res);
}
