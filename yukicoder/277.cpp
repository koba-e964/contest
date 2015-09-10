#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;

const int N = 100100;

VI edges[N];

int rd[N], ld[N];

VI leaves;

void dfs(int v, int par = -1) {
  if (par == -1) {
    rd[v] = 0;
  } else {
    rd[v] = rd[par] + 1;
  }
  int child = 0;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    dfs(w, v);
    child++;
  }
  if (child == 0) {
    leaves.push_back(v);
  }
}


void bfs(void) {
  queue<PI> que;
  REP(i, 0, leaves.size()) {
    que.push(PI(leaves[i], 0));
  }
  while (!que.empty()) {
    PI t = que.front(); que.pop();
    int x = t.first;
    int d = t.second;
    if (ld[x] >= 0) {
      continue;
    }
    REP(i, 0, edges[x].size()) {
      int y = edges[x][i];
      que.push(PI(y, d + 1));
    }
    ld[x] = d;
  }
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    edges[x].push_back(y);
    edges[y].push_back(x);
  }
  REP(i, 0, n) {
    rd[i] = -1;
    ld[i] = -1;
  }
  dfs(0);
  bfs();
  REP(i, 0, n) {
    cout << min(rd[i], ld[i]) << endl;
  }
}
