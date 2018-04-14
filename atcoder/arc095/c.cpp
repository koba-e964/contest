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
  vector<PI> a(n);
  REP(i, 0, n) {
    int v;
    cin >> v;
    a[i] = PI(v, i);
  }
  sort(a.begin(), a.end());
  VI ans(n);
  REP(i, 0, n) {
    int idx = a[i].second;
    if (i < n / 2) {
      ans[idx] = a[n / 2].first;
    } else {
      ans[idx] = a[n / 2 - 1].first;
    }
  }
  REP(i, 0, n) {
    cout << ans[i] << endl;
  }
}
