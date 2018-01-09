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
  REP(i, 0, n) {
    cin >> a[i];
  }
  int inv = 0;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      if (a[i] > a[j]) { inv = 1 - inv; }
    }
  }
  int m;
  cin >> m;
  REP(i, 0, m) {
    int l, r;
    cin >> l >> r;
    int len = r - l + 1;
    inv += len * (len - 1) / 2;
    inv %= 2;
    cout << (inv == 0 ? "even" : "odd") << "\n";
  }
}
