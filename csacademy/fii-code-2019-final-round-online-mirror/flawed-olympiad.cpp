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
  cout << "impossible" << endl;
  exit(0);
}

const int N = 100100;
ll a[N];

bool dfs(int i, ll cur, ll mask, VI &ans) {
  if (i < 0) {
    return (cur & mask) == (a[0] & mask);
  }
  // cerr << "cur,mask = " << cur << " " << mask << endl;
  if ((cur & a[i + 1] & mask) == (cur & mask)) {
    mask &= a[i + 1];
    ans[i] = 1;
    if (dfs(i - 1, cur, mask, ans)) return 1;
  }
  if (((cur | a[i + 1]) & mask) == (cur & mask)) {
    mask &= ~a[i + 1];
    ans[i] = 2;
    if (dfs(i - 1, cur, mask, ans)) return 1;
  }
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll x;
  cin >> n >> x;
  REP(i, 0, n) cin >> a[i];
  VI ans(n - 1);
  ll cur = x;
  ll mask = (1LL << 30) - 1;
  bool res = dfs(n - 2, cur, mask, ans);
  if (!res) fail();
  REP(i, 0, n - 1) {
    cout << a[i] << (ans[i] == 1 ? "&" : "|");
  }
  cout << a[n - 1] << "\n";
}
