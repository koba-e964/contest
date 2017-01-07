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

const int N = 110;
int dist[N][N];

int main(void){
  int n, m;
  cin >> n >> m;
  const int inf = 1e8;
  REP(i, 0, N) {
    REP(j, 0, N) {
      dist[i][j] = inf;
    }
    dist[i][i] = 0;
  }
  int cnt = 0;
  typedef pair<int, PI> PIII;
  vector<PIII> edges;
  REP(i, 0, m) {
    int a, b, c;
    cin >> a >> b >> c;
    a--, b--;
    edges.push_back(PIII(c, PI(a, b)));
  }
  sort(edges.begin(), edges.end());
  REP(i, 0, m) {
    // update
    PIII t = edges[i];
    int a = t.second.first;
    int b = t.second.second;
    int c = t.first;
    if (dist[a][b] < c) {
      cnt++;
    } else {
      dist[a][b] = c;
      dist[b][a] = c;
      REP(j, 0, n) {
	REP(k, 0, n) {
	  dist[j][k] = min(dist[j][k], dist[j][a] + dist[a][k]);
	}
      }
      REP(j, 0, n) {
	REP(k, 0, n) {
	  dist[j][k] = min(dist[j][k], dist[j][b] + dist[b][k]);
	}
      }
    }
    if (0) {
      cerr << "dist:" << endl;
      REP(j, 0, n) {
	cerr << "[" << j << "]";
	REP(k, 0, n) {
	  cerr << " " << dist[j][k];
	}
	cerr << endl;
      }
    }
  }
  cout << cnt << endl;
}
