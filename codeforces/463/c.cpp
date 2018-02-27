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
  int n, a, b;
  cin >> n >> a >> b;
  REP(i, 0, n + 1) {
    if ((ll)i * a > n) continue;
    int j = n - i * a;
    if (j % b != 0) continue;
    j /= b;
    // construct
    VI ans(n);
    REP(k, 0, i) {
      int st = a * k;
      REP(l, 0, a) {
	ans[st + l] = st + (l + 1) % a;
      }
    }
    REP(k, 0, j) {
      int st = a * i + b * k;
      REP(l, 0, b) {
	ans[st + l] = st + (l + 1) % b;
      }
    }
    REP(i, 0, n) {
      cout << ans[i] + 1 << (i == n - 1 ? "\n" : " ");
    }
    return 0;
  }
  cout << -1 << "\n";
}
