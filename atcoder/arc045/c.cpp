#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 1e5 + 10;

vector<PI> edges[N];

int cost[N];

void dfs(int v, int sum) {
  if (cost[v] >= 0) {
    return;
  }
  cost[v] = sum;
  REP(i, 0, edges[v].size()) {
    PI w = edges[v][i];
    dfs(w.first, sum ^ w.second);
  }
}

int main(void){
  int n, xx;
  cin >> n >> xx;
  REP(i, 0, n - 1) {
    int x, y, c;
    cin >> x >> y >> c;
    x--, y--;
    edges[x].push_back(PI(y, c));
    edges[y].push_back(PI(x, c));
  }
  REP(i, 0, n) {
    cost[i] = -1;
  }
  dfs(0, 0);
  map<int, int> tc;
  REP(i, 0, n) {
    if (tc.count(cost[i]) == 0) {
      tc[cost[i]] = 1;
    } else {
      tc[cost[i]] ++;
    }
  }
  ll cnt = 0;
  REP(i, 0, n) {
    if (tc.count(xx ^ cost[i])) {
      cnt += tc[xx ^ cost[i]];
    }
  }
  if (xx == 0) {
    cnt -= n;
  }
  cout << cnt / 2 << endl;
}
