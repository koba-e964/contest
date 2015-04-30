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

const int N = 210;

int dist[N][N];

int orig[N][N];

int n;
void trace(int s, int g) {
  if (s == g) {
    cout << g << endl;
    return;
  }
  REP(i, 0, n) {
    if (dist[s][g] == dist[i][g] + orig[s][i]) {
      cout << s << " ";
      trace(i, g);
      return;
    }
  }
  assert(0);
}

int main(void){
  const int inf = 0x3fffff;
  int m, s, g;
  cin >> n >> m >> s >> g;
  REP(i,0,n) {
    REP(j,0,n) {
      dist[i][j] = orig[i][j] = inf;
    }
    dist[i][i] = 0;
  }
  REP(i, 0, m) {
    int a, b, c;
    cin >> a >> b >> c;
    dist[a][b] = orig[a][b] = c;
    dist[b][a] = orig[b][a] = c;
  }
  REP(k, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  trace(s, g);
}
