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
  int n, m;
  cin >> n >> m;
  vector<VI> r(n, VI(m));
  REP(i, 0, n) REP(j, 0, m) cin >> r[i][j];
  VI ma(m, -1);
  vector<VI> ms(m);
  VI pop(n);
  int ans = 0;
  REP(i, 0, n) {
    REP(j, 0, m) {
      int val = r[i][j];
      if (ma[j] < val) {
        ma[j] = val;
        for (int e: ms[j]) {
          pop[e]--;
          if (pop[e] == 0) {
            ans--;
          }
        }
        ms[j].clear();
      }
      if (ma[j] == val) {
        ms[j].push_back(i);
        pop[i]++;
        if (pop[i] == 1) ans++;
      }
    }
    cout << ans << endl;
  }
}
