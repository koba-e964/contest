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

int gcd(int x, int y) {
  while(y != 0) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int l, r, d;
  cin >> l >> r >> d;
  int g = 0;
  VI ans;
  REP(i, l, r + 1) {
    if (i % d == 0) {
      ans.push_back(i);
      g = gcd(g, i);
    }
  }
  if (g != d) {
    cout << "impossible\n";
  } else {
    cout << ans.size() << "\n";
    REP(i, 0, ans.size()) {
      cout << ans[i] << (i + 1 == ans.size() ? "\n" : " ");
    }
  }
}
