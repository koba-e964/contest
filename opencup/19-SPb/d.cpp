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
typedef pair<ll, ll> PI;

ll nearest(ll a, ll b) {
  a += b / 2;
  ll r = a % b;
  if (r < 0) r += b;
  a -= r;
  return a / b;
}

PI nn(ll a, ll b) {
  ll c = nearest(2 * a + b, 5);
  ll d = nearest(2 * b - a, 5);
  return PI(c, d);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll a, b, c, d;
  cin >> a >> b >> c >> d;
  PI x = nn(a, b);
  PI y = nn(c, d);
  cout << abs(x.first - y.first) + abs(x.second - y.second) << endl;
}
