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
typedef pair<ll, int> PLI;

// Greedily picks the last elements
int calc(const vector<PLI> &pool, ll x, int en) {
  for (int i = en - 1; i >= 0; --i) {
    ll top = pool[i].first * (en - i);
    if (x <= top) {
      return i;
    }
  }
  return -1;
}

void output(const VI &ans1, const VI &ans2) {
  cout << "Yes" << "\n";
  cout << ans1.size() << " " << ans2.size() << "\n";
  REP(i, 0, ans1.size()) cout << ans1[i] << (i == (int) ans1.size() - 1 ? "\n" : " ");
  REP(i, 0, ans2.size()) cout << ans2[i] << (i == (int) ans2.size() - 1 ? "\n" : " ");
  exit(0);
}

void fail(void) {
  cout << "No\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll x1, x2;
  cin >> n >> x1 >> x2;
  VL c(n);
  REP(i, 0, n) cin >> c[i];
  vector<PLI> pool;
  REP(i, 0, n) pool.push_back(PLI(c[i], i));
  sort(pool.begin(), pool.end());

  // x1 first
  REP(tempo, 0, 2) {
    int en1 = calc(pool, x1, n);
    if (en1 == -1) {
      swap(x1, x2);
      continue;
    }
    int en2 = calc(pool, x2, en1);
    if (en2 == -1) {
      swap(x1, x2);
      continue;
    }
    VI ans1, ans2;
    REP(i, en2, en1) ans2.push_back(pool[i].second + 1);
    REP(i, en1, n) ans1.push_back(pool[i].second + 1);
    if (tempo == 1) swap(ans1, ans2);
    output(ans1, ans2);
  }
  fail();
}
