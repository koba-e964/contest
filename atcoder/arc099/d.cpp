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

int s(ll n) {
  int sum = 0;
  while (n > 0) {
    sum += n % 10;
    n /= 10;
  }
  return sum;
}

void brute(void) {
  int n = 1000000;
  vector<double> t(n);
  REP(i, 1, n) {
    t[i] = (double) i / (double) s(i);
  }
  REP(i, 1, n) {
    bool ok = true;
    REP(j, i + 1, n) {
      if (t[j] <= t[i] - 1e-11) {
        ok = false;
        if (i == 419999) {
          DEBUGP(i);
          DEBUGP(j);
          DEBUGP(t[i]);
          DEBUGP(t[j]);
        }
        break;
      }
    }
    if (ok) {
      cerr << i << " " << t[i] << endl;
    }
  }
}

typedef pair<ll, double> PLD;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll k;
  cin >> k;
  ll cur = 1;
  VL pool;
  REP(tap, 0, 15) {
    REP(i, 1, 1000) {
      ll v = i * cur + cur - 1;
      if (v <= 1e15)
        pool.push_back(v);
    }
    cur *= 10;
  }
  sort(pool.begin(), pool.end());
  pool.erase(unique(pool.begin(), pool.end()), pool.end());
  VL meguru(pool.size());
  REP(i, 0, pool.size()) meguru[i] = s(pool[i]);
  VL ans;
  REP(i, 0, pool.size()) {
    bool ok = 1;
    REP(j, i + 1, pool.size()) {
      if (meguru[j] * pool[i] > pool[j] * meguru[i]) {
        ok = 0;
        break;
      }
    }
    if (ok) {
      ans.push_back(pool[i]);
    }
  }
  REP(i, 0, k) cout << ans[i] << endl;
}
