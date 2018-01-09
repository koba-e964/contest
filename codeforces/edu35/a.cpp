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
  VI a(n);
  int mi = 1.1e9;
  REP(i, 0, n) {
    cin >> a[i];
    mi = min(mi, a[i]);
  }
  VI occ;
  REP(i, 0, n) {
    if (a[i] == mi) {
      occ.push_back(i);
    }
  }
  int ans = 1.1e9;
  REP(i, 0, occ.size() - 1) {
    ans = min(ans, occ[i + 1] - occ[i]);
  }
  cout << ans << "\n";
}
