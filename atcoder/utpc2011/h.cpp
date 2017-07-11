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

int m, n, k;
VI w, a;
map<pair<int, VI>, int > memo;

int rec(int v, VI st) {
  if (v >= k) {
    return 0;
  }
  if (memo.count(make_pair(v, st))) {
    return memo[make_pair(v, st)];
  }
  memo[make_pair(v, st)] = 0;
  int &ret = memo[make_pair(v, st)];
  REP(i, 0, m) {
    if (st[i] == a[v]) {
      return ret = rec(v + 1, st);
    }
  }
  int mi = 1e8;
  REP(i, 0, m) {
    VI stc(st);
    stc[i] = a[v];
    mi = min(mi, rec(v, stc));
  }
  return ret = mi + w[a[v]];
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> m >> n >> k;
  REP(i, 0, n) {
    int t;
    cin >> t;
    w.push_back(t);
  }
  REP(i, 0, k) {
    int t;
    cin >> t;
    a.push_back(t);
    a[i]--;
  }
  cout << rec(0, VI(m, -1)) << endl;
}
