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

PI solve(int cur, const VI &pts) {
  int idx = lower_bound(pts.begin(), pts.end(), cur) - pts.begin();
  if (idx == 0) {
    return PI(pts[pts.size() - 1], 0);
  }
  return PI(pts[idx - 1], 1);
}

int main(void){
  int n;
  cin >> n;
  map<int, VI> p;
  REP(i, 0, n) {
    int a;
    cin >> a;
    if (not p.count(a)) {
      p[a] = VI();
    }
    p[a].push_back(i);
  }
  int cnt = 0;
  int cur = 0;
  for (map<int, VI>::iterator it = p.begin(); it != p.end(); ++it) {
    PI res = solve(cur, it->second);
    cur = res.first;
    cnt += res.second;
  }
  if (cur != 0) {
    cnt++;
  }
  cout << cnt << endl;
}
