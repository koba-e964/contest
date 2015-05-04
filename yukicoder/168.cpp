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
typedef pair<ll, int> PI;
const double EPS=1e-9;

const int N = 1010;

ll x[N], y[N];

ll dist[N][N];

ll d0[N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  double m = 0;
  REP(i, 0, n) {
    REP(j, 0, n) {
      ll sq = (x[j] - x[i]) * (x[j] - x[i]) + (y[j] - y[i]) * (y[j] - y[i]);
      sq = (sq + 99) / 100;
      dist[i][j] = ceil(sqrt((long double)sq));
    }
  }
  priority_queue<PI, vector<PI>, greater<PI> > que;
  que.push(PI(0, 0));
  const ll inf = 123456789LL << 32;
  REP(i, 0, n) {
    d0[i] = inf;
  }
  while (! que.empty()) {
    PI p = que.top(); que.pop();
    if (p.first >= d0[p.second]) {
      continue;
    }
    d0[p.second] = p.first;
    REP(i, 0, n) {
      ll q = max(p.first, dist[p.second][i]);
      if (q < d0[i]) {
        que.push(PI(q, i));
      }
    }
  }
  cout << 10 * d0[n-1] << endl;
}
