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

const int N = 100100;
ll x[N], y[N];

int main(void){
  int n, m;
  cin >> n >> m;
  ll tot = 0;
  VL d(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
    d[i] = x[i] - y[i];
  }
  ll ma = 0;
  priority_queue<ll, vector<ll>, greater<ll> > que;
  ll que_tot = 0;
  REP(i, 0, n) {
    tot += y[i];
    ma = max(ma, tot + que_tot + d[i]);
    que.push(d[i]);
    que_tot += d[i];
    if (que.size() > m - 1) {
      ll t = que.top(); que.pop();
      que_tot -= t;
    }
  }
  cout << ma << endl;
}
