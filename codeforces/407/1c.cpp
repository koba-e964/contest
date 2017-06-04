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

const int BIAS = 1100;
const int N = 2 * BIAS + 1;
const int inf = 1e8;

int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n, k;
  cin >> n >> k;
  VI a(k);
  REP(i, 0, k) {
    cin >> a[i];
    a[i] -= n;
  }
  set<int> uniq(a.begin(), a.end());
  vector<int> b(uniq.begin(), uniq.end());
  sort(b.begin(), b.end());
  k = b.size(); // k <= 1000
  queue<PI> que;
  que.push(PI(BIAS, 0));
  VI dist(N, inf);
  while (not que.empty()) {
    PI t = que.front(); que.pop();
    int v = t.first;
    int d = t.second;
    if (d > 0) {
      if (dist[v] <= d) { continue; }
      dist[v] = d;
    }
    REP(i, 0, k) {
      int nv = v + b[i];
      if (nv < 0 || nv >= N) { continue; }
      que.push(PI(nv, d + 1));
    }
  }
  cout << (inf == dist[BIAS] ? -1 : dist[BIAS]) << endl;
}
