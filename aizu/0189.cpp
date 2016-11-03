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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 15;
int inf = 0x3fffffff;
int dist[N][N];

int main(void){
  int n;
  while(cin >> n) {
    int nt = 0;
    if (n == 0) { break; }
    REP(i, 0, N) {
      REP(j, 0, N) {
	dist[i][j] = i == j ? 0 : inf;
      }
    }
    REP(i, 0, n) {
      int a, b, c;
      cin >> a >> b >> c;
      nt = max(nt, max(a, b) + 1);
      dist[a][b] = min(dist[a][b], c);
      dist[b][a] = min(dist[b][a], c);
    }
    REP(k, 0, nt) {
      REP(i, 0, nt) {
	REP(j, 0, nt) {
	  dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
	}
      }
    }
    int mi = inf, mini = -1;
    REP(i, 0, n) {
      int tot = 0;
      REP(j, 0, nt) {
	tot += dist[i][j];
      }
      if (mi > tot) {
	mi = tot;
	mini = i;
      }
    }
    cout << mini << " " << mi << endl;
  }
}
