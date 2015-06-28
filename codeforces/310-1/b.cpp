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

const int N = 2e5 + 10;
ll l[N], r[N];

typedef pair<ll, ll> PL;
PL a[N];
pair<PL,int> dist[N];
int sol[N];
int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> l[i] >> r[i];
  }
  REP(i, 0, m) {
    ll q;
    cin >> q;
    a[i] = PL(q, i);
  }
  REP(i, 0, n - 1) {
    dist[i] = pair<PL,int>(PL(l[i + 1] - r[i], r[i + 1] - l[i]), i);
  }
  sort(dist, dist + n - 1);
  sort(a, a + m);
  int pos = 0;
  int ai = 0;
  priority_queue<PL, vector<PL>, greater<PL> > que;
  while (ai < m) {
    PL c = a[ai];
    PL p = dist[pos].first;
    while (pos < n - 1 && dist[pos].first.first <= c.first) {
      que.push(PL(dist[pos].first.second, dist[pos].second));
      pos++;
    }
    if (que.empty()) {
      ai++;
      continue;
    }
    PL top = que.top();
    if (c.first <= top.first) {
      sol[top.second] = c.second + 1;
      ai++;
      que.pop();
      continue;
    }
    break;
  }
  if (pos == n - 1 && que.empty()) {
    cout << "Yes" << endl;
    REP(i, 0, n - 1) {
      cout << sol[i];
      if (i < n - 1) cout << " ";
    }
    cout << endl;
  } else {
    cout << "No" << endl;
  }
}
