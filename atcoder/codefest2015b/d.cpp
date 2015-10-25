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

const int N = 100100;
ll s[N], c[N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> s[i] >> c[i];
  }
  vector<ll> res(n);
  typedef pair<int, ll> PIL;
  vector<pair<ll, int> > tsk(n);
  REP(i, 0, n) {
    tsk[i] = pair<ll, int>(s[i], i);
  }
  sort(tsk.begin(), tsk.end());
  priority_queue<PIL, vector<PIL>, greater<PIL> > que;
  ll t = tsk[0].first;
  int pos = 0;
  while(pos < n || ! que.empty()) {
    while (pos < n && tsk[pos].first <= t) {
      assert (tsk[pos].first == t);
      int idx = tsk[pos].second;
      que.push(PIL(idx, c[idx]));
      pos++;
    }
    if (que.empty()) {
      t = tsk[pos].first;
      continue;
    }
    PIL p = que.top(); que.pop();
    ll sup = 1e17;
    if (pos < n) {
      sup = tsk[pos].first;
    }
    if (sup >= t + p.second) {
      res[p.first] = t + p.second - 1;
      t = t + p.second;
    } else {
      que.push(PIL(p.first, p.second - sup + t));
      t = sup;
    }
  }
  REP(i, 0, n) {
    cout << res[i] << endl;
  }
}
