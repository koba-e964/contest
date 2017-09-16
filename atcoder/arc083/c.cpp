#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


int main(void) {
  int a, b, c, d, e, f;
  cin >> a >> b >> c >> d >> e >> f;
  vector<bool> wat(f + 1, false);
  queue<int> que;
  que.push(0);
  while (not que.empty()) {
    int t = que.front(); que.pop();
    if (t > f) { continue; }
    if (wat[t]) { continue; }
    wat[t] = true;
    que.push(t + 100 * a);
    que.push(t + 100 * b);
  }
  vector<bool> sug(f + 1, false);
  que.push(0);
  while (not que.empty()) {
    int t = que.front(); que.pop();
    if (t > f) { continue; }
    if (sug[t]) { continue; }
    sug[t] = true;
    que.push(t + c);
    que.push(t + d);
  }
  pair<double, PI> ma(-5e15, PI(0, 0));
  REP(i, 1, f + 1) {
    if (not wat[i]) { continue; }
    REP(j, 0, f - i + 1) {
      if (not sug[j]) { continue; }
      if (i * e < j * 100) { continue; }
      ma = max(ma, make_pair(j * 100.0 / i / e, PI(i + j, j)));
    }
  }
  cout << ma.second.first << " " << ma.second.second << endl;
}
