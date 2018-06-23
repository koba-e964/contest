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
  int n, k;
  cin >> n >> k;
  VI a(n);
  int idx = -1;
  REP(i, 0, n) {
    cin >> a[i];
    if (a[i] == 1) idx = i;
  }
  int r1 = idx % (k - 1);
  int r2 = (n - idx - 1) % (k - 1);
  int ans = idx / (k - 1) + (n - idx - 1) / (k - 1);
  if (r1 + r2 > 0) ans++;
  if (r1 + r2 >= k) ans++;
  cout << ans << endl;
}
