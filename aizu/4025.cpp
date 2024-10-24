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
  VI a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  vector<PI> ba;
  REP(i, 0, n) {
    ba.push_back(PI(b[i], a[i]));
  }
  sort(ba.begin(), ba.end());
  int ans = 0;
  REP(i, 0, n) {
    if (ba[i].first == ba[i].second) {
      continue;
    }
    bool found = false;
    REP(j, i + 1, n) {
      if (ba[i].second == ba[j].first) {
        found = true;
      }
    }
    if (!found || ba[i].first > ba[i].second) {
      cout << -1 << endl;
      return 0;
    }
    ans++;
  }
  cout << ans << endl;
}
