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


const int N = 55;
const int inf = 123456789;
int s[N];
int dist[N][N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> s[i];
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      dist[i][j] = inf;
    }
    dist[i][i] = 0;
  }
  int m;
  cin >> m;
  REP(i, 0, m) {
    int a, b, c;
    cin >> a >> b >> c;
    dist[a][b] = c;
    dist[b][a] = c;
  }
  REP(k, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  m = inf;
  REP(i, 1, n - 1) {
    REP(j, 1, n - 1) {
      if (i != j) {
	m = min(m, dist[0][i] + dist[i][j] + dist[j][n - 1] + s[i] + s[j]);
      }
    }
  }
  cout << m << endl;
}
