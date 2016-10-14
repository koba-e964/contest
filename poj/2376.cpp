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
const int N = 250010;
PI a[N];

int main(void){
  int n, t;
  cin >> n >> t;
  REP(i, 0, n) {
    int x, y;
    cin >> x >> y;
    a[i] = PI(x, y);
  }
  sort(a, a + n);
  int cnt = 0, cur = 0, pos = 0;
  while (pos < n || cur < t) {
    vector<PI> que;
    while (pos < n && a[pos].first <= cur + 1) {
      que.push_back(PI(a[pos].second, a[pos].first));
      pos++;
    }
    if (que.empty()) {
      break;
    }
    sort(que.rbegin(), que.rend());
    PI p = que[0];
    if (cur < p.first) {
      cur = p.first;
      cnt++;
    }
    // cerr << "cur = " << cur << ", cnt = " << cnt << endl;
  }
  if (cur >= t) {
    cout << cnt << endl;
  } else {
    cout << -1 << endl;
  }
}
