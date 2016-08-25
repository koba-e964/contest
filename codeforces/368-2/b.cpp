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



int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  vector<pair<ll, ll>  > e;
  REP(i, 0, m) {
    ll u, v, l;
    cin >> u >> v >> l;
    u--, v--;
    e.push_back(pair<ll, ll>(l, (u << 32) + v));
  }
  vector<bool> a(n);
  REP(j, 0, k) {
    int t;
    cin >> t;
    a[t - 1] = true;
  }

  sort(e.begin(), e.end());
  REP(i, 0, e.size()) {
    pair<ll, ll> q = e[i];
    int u = q.second >> 32;
    int v = q.second;
    if (a[u] ^ a[v]) {
      cout << q.first << endl;
      return 0;
    }
  }
  cout << -1 << endl;
  return 0;
}
