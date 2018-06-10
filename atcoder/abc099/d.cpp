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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, cc;
  cin >> n >> cc;
  vector<VI> d(cc, VI(cc));
  REP(i, 0, cc) REP(j, 0, cc) cin >> d[i][j];
  vector<VI> c(n, VI(n));
  REP(i, 0, n) REP(j, 0, n) {
    cin >> c[i][j];
    c[i][j]--;
  }
  ll mi = 1e16;
  vector<VI> homo(3, VI(cc, 0));
  REP(i, 0, n) {
    REP(j, 0, n) {
      homo[(i + j) % 3][c[i][j]] += 1;
    }
  }
  REP(i, 0, cc) {
    REP(j, 0, cc) {
      if (i == j) continue;
      REP(k, 0, cc) {
        if (i == k || j == k) continue;
        int sapiens[3] = {i, j, k};
        ll cost = 0;
        REP(q, 0, 3) {
          REP(u, 0, cc) {
            cost += (ll) homo[q][u] * (ll) d[u][sapiens[q]];
          }
        }
        mi = min(mi, cost);
      }
    }
  }
  cout << mi << endl;
}
