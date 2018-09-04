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
  ll s;
  cin >> n >> s;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VI gt, lt;
  REP(i, 0, n) {
    if (a[i] < s) lt.push_back(a[i]);
    if (a[i] > s) gt.push_back(a[i]);
  }
  sort(lt.begin(), lt.end());
  sort(gt.rbegin(), gt.rend());
  ll tot = 0;
  REP(i, n / 2, lt.size()) {
    tot += s - lt[i];
  }
  REP(i, n / 2, gt.size()) {
    tot += gt[i] - s;
  }
  cout << tot << endl;
}
