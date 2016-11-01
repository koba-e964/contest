#include <algorithm>
#include <bitset>
#include <cassert>
#include <cstdio>
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

typedef pair<PI, int> PPII;

bool find_negative_loop(int n, const vector<PPII> &es) {
  VI dist(n, 0);
  REP(i, 0, n) {
    REP(j, 0, es.size()) {
      PPII edge = es[j];
      int u = edge.first.first, v = edge.first.second;
      int cost = edge.second;
      if (dist[v] > dist[u] + cost) {
	dist[v] = dist[u] + cost;
	if (i == n - 1) {
	  return true;
	}
      }
    }
  }
  return false;
}

int main(void){
  int f;
  scanf("%d", &f);
  REP(loop_cnt, 0, f) {
    int n, m, w;
    scanf("%d%d%d", &n, &m, &w);
    vector<PPII> es(2 * m + w);
    REP(i, 0, m + w) {
      scanf("%d%d%d", &es[i].first.first, &es[i].first.second, &es[i].second);
      es[i].first.first--;
      es[i].first.second--;
      if (i >= m) {
	es[i].second *= -1;
      } else {
	es[i + m + w] = es[i];
	swap(es[i].first.first, es[i].first.second);
      }
    }
    puts(find_negative_loop(n, es) ? "YES" : "NO");
  }
}
