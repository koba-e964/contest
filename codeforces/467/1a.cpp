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
  ll k, d, t;
  cin >> k >> d >> t;
  ll du = (k + d - 1) / d * d;
  ll per = k + du;
  ll q = (2 * t) / per;
  ll r = (2 * t) % per;
  if (r <= 2 * k) {
    printf("%.15f\n", (double) q * (double) du + r / 2.0);
  } else {
    printf("%.15f\n", (double) q * (double) du + (r - 2 * k) + k);
  }
}
