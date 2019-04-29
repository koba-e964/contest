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

ll ask(int d) {
  cout << d << endl;
  ll x;
  cin >> x;
  return x;
}

int main(void) {
  int t, w;
  cin >> t >> w;
  assert (w >= 2);
  REP(_, 0, t) {
    VL d(6);
    ll a = ask(210);
    d[5] = (a >> 35) & 127;
    d[4] = (a >> 42) & 1023;
    d[3] = a >> 52;
    ll b = ask(42);
    REP(i, 3, 6) {
      b -= d[i] << (42 / (i + 1));
    }
    d[2] = (b >> 14) & 127;
    d[1] = (b >> 21) & 1023;
    d[0] = b >> 42;
    REP(i, 0, 6) cout << d[i] << (i == 5 ? "" : " ");
    cout << endl;
    int resp;
    cin >> resp;
    if (resp == -1) {
      return 1;
    }
  }
}
