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
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    VI a(n);
    REP(i, 0, n) cin >> a[i];
    int sum = 0;
    vector<PI> t;
    REP(i, 0, n) {
      sum += a[i];
      t.push_back(PI(a[i], i));
    }
    if (sum != 2 * n - 2) {
      cout << "invalid" << endl;
      continue;
    }
    sort(t.begin(), t.end());
    int pos = 0;
    vector<PI> e;
    REP(i, 0, n - 2) {
      while (t[pos].first < 2) pos++;
      e.push_back(PI(t[i].second, t[pos].second));
      t[pos].first -= 1;
    }
    e.push_back(PI(t[n - 2].second, t[n - 1].second));
    assert ((int) e.size() == n - 1);
    cout << n - 1 << endl;
    REP(i, 0, n - 1) {
      cout << e[i].second + 1 << " " << e[i].first + 1 << "\n";
    }
  }
}
