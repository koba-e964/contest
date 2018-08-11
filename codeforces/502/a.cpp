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
  vector<PI> pool;
  REP(i, 0, n) {
    int a, b, c, d;
    cin >> a >> b >> c >> d;
    pool.push_back(PI(a + b + c + d, -i));
  }
  sort(pool.rbegin(), pool.rend());
  int ans = -1;
  REP(i, 0, n) {
    if (pool[i].second == 0) {
      ans = i;
    }
  }
  cout << ans + 1 << endl;
}
