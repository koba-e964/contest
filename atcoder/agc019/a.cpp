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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ll q, h, s, d, n;
  cin >> q >> h >> s >> d >> n;
  h = min(h, 2 * q);
  s = min(s, 2 * h);
  d = min(d, 2 * s);
  n *= 4;
  ll tot = 0;
  tot += n / 8 * d; n %= 8;
  tot += n / 4 * s; n %= 4;
  tot += n / 2 * h; n %= 2;
  tot += n * q;
  cout << tot << endl;
}
