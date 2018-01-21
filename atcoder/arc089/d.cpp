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

const int N = 4010;

int tbl[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VI x(n), y(n);
  REP(i, 0, n) {
    string cl;
    cin >> x[i] >> y[i] >> cl;
    x[i] %= 2 * k;
    y[i] %= 2 * k;
    if (cl == "W") {
      y[i] = (y[i] + k) % (2 * k);
    }
  }
  REP(i, 0, n) {
    tbl[x[i]][y[i]] += 1;
    tbl[x[i]][y[i] + k] -= 1;
    tbl[x[i] + k][y[i]] -= 1;
    tbl[x[i] + k][y[i] + k] += 1;
    x[i] += k;
    y[i] += k;
    tbl[x[i]][y[i]] += 1;
    tbl[x[i]][y[i] + k] -= 1;
    tbl[x[i] + k][y[i]] -= 1;
    tbl[x[i] + k][y[i] + k] += 1;
  }
  REP(i, 0, N) {
    REP(j, 0, N) {
      if (i + j == 0) continue;
      if (i > 0) {
	tbl[i][j] += tbl[i - 1][j];
      }
      if (j > 0) {
	tbl[i][j] += tbl[i][j - 1];
      }
      if (i > 0 && j > 0) {
	tbl[i][j] -= tbl[i - 1][j - 1];
      }
    }
  }
  REP(i, 0, 4 * k) {
    REP(j, 0, 4 * k) {
      int c = tbl[i][j];
      tbl[i][j] = 0;
      tbl[i % (2 * k)][j % (2 * k)] += c;
    }
  }
  int ma = 0;
  REP(i, 0, 2 * k) {
    REP(j, 0, 2 * k) {
      ma = max(ma, tbl[i][j]);
    }
  }
  cout << ma << endl;
}
