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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  map<ll, ll> tbl;
  REP(i, 0, n) {
    ll a, x;
    cin >> a >> x;
    tbl[a] = max(tbl[a], x);
  }
  int m;
  cin >> m;
  REP(i, 0, m) {
    ll b, y;
    cin >> b >> y;
    tbl[b] = max(tbl[b], y);
  }
  ll sum = 0;
  for (auto p: tbl) {
    sum += p.second;
  }
  cout << sum << endl;
}
