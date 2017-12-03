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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int k;
  ll p;
  cin >> k >> p;
  VL pool;
  REP(i, 1, k + 1) {
    ll v = i;
    ll w = 0;
    ll bias = 1;
    while (v > 0) {
      ll r = v % 10;
      v /= 10;
      w = 10 * w + r;
      bias *= 10;
    }
    pool.push_back(i * bias + w);
  }
  ll tot = 0;
  REP(i, 0, k) {
    tot = (tot + pool[i]) % p;
  }
  cout << tot << "\n";
}
