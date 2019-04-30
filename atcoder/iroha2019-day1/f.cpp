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

void fail(void) {
  cout << -1 << endl;
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n, k;
  cin >> n >> k;
  if (k >= 2000) fail();
  ll p = 2;
  VL ans(k);
  int ptr = 0;
  while (k > 1) {
    while (n % p != 0) {
      if (p * p > n) fail();
      p++;
    }
    n /= p;
    ans[ptr++] = p;
    k--;
  }
  ans[ptr] = n;
  REP(i, 0, ans.size()) cout << ans[i] << (i == ans.size() - 1 ? "\n" : " ");
}
