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
  vector<string> s(n);
  VL coef(10);
  vector<bool> non_zero(10);
  REP(i, 0, n) {
    cin >> s[i];
    ll cur = 1;
    REP(j, 0, s[i].length()) {
      coef[s[i][s[i].length() - 1 - j] - 'a'] += cur;
      cur *= 10;
    }
    non_zero[s[i][0] - 'a'] = true;
  }
  pair<ll, int> zero(-1, -1);
  REP(i, 0, 10) {
    if (not non_zero[i]) {
      zero = max(zero, make_pair(coef[i], i));
    }
  }
  int z = zero.second;
  vector<pair<ll, int> > pool;
  REP(i, 0, 10) {
    if (i != z) {
      pool.push_back(make_pair(coef[i], i));
    }
  }
  sort(pool.begin(), pool.end());
  ll tot = 0;
  REP(i, 0, 9) {
    tot += pool[i].first * (9 - i);
  }
  cout << tot << "\n";
}
