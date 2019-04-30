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
  ll n, a, b;
  cin >> n >> a >> b;
  VL d(b);
  REP(i, 0, b) cin >> d[i];
  d.push_back(0);
  sort(d.begin(), d.end());
  d.push_back(n + 1);
  ll tot = 0;
  REP(i, 0, d.size() - 1) {
    ll u = d[i], v = d[i + 1];
    ll date = (v - u - 1) / a;
    tot += v - u - 1 - date;
  }
  cout << tot << endl;
}
