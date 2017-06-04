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
  int n, k;
  cin >> n >> k;
  VL c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  VI cap(k);
  vector<bool> is_cap(n, false);
  REP(i, 0, k) {
    cin >> cap[i];
    cap[i]--;
    is_cap[cap[i]] = true;
  }
  ll tot = 0;
  ll btot = 0;
  ll ctot = 0;
  ll csqtot = 0;
  REP(i, 0, n) {
    tot += 2 * c[i] * c[(i + 1) % n];
    btot += c[i];
  }
  REP(i, 0, k) {
    ctot += c[cap[i]];
    csqtot += c[cap[i]] * c[cap[i]];
  }
  REP(i, 0, k) {
    tot += 2 * btot * c[cap[i]];
    int u = (cap[i] + n - 1) % n;
    int v = (cap[i] + 1) % n;
    
    tot -= (is_cap[u] ? 1 : 2) * c[cap[i]] * c[u];
    tot -= (is_cap[v] ? 1 : 2) * c[cap[i]] * c[v];
  }
  // remove cap
  tot -= ctot * ctot - csqtot;
  tot -= 2 * csqtot;
  cout << tot / 2 << endl;
}
