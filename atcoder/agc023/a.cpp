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
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VL acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + a[i];
  map<ll, int> tbl;
  REP(i, 0, n + 1) tbl[acc[i]]++;
  ll tot = 0;
  for (auto &e: tbl) {
    ll x = e.second;
    tot += x * (x - 1) / 2;
  }
  cout << tot << endl;
}
