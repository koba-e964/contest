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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 1010;


struct edge {
  int to, size;
};

vector<edge> dep[N];

int n, m;
int memo[N][N];


void rec(int x) {
  if (memo[x][0] >= 0) {
    return;
  }
  if (dep[x].size() == 0) {
    REP(i, 0, n) {
      memo[x][i] =0;
    }
    memo[x][x] = 1;
    return;
  }
  REP(j, 0, n) {
    memo[x][j] = 0;
  }
  REP(i, 0, dep[x].size()) {
    int to = dep[x][i].to;
    int size = dep[x][i].size;
    rec(to);
    REP(j, 0, n) {
      memo[x][j] += memo[to][j] * size;
    }
  }
  return;
}

int main(void){
  cin >> n >> m;
  REP(i, 0, m) {
    int p, q, r;
    cin >> p >> q >> r;
    --p, --r;
    dep[r].push_back((edge) {p, q});
  }
  REP(i, 0, n) {
    memo[i][0] = -1;
  }
  rec(n - 1);
  REP(i, 0, n - 1) {
    cout << memo[n - 1][i] << endl;
  }
}
