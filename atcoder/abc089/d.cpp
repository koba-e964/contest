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
  int h, w, d;
  cin >> h >> w >> d;
  vector<VI> a(h, VI(w));
  vector<PI> t(h * w);
  REP(i, 0, h) {
    REP(j, 0, w) {
      cin >> a[i][j];
      a[i][j]--;
      t[a[i][j]] = PI(i, j);
    }
  }
  vector<ll> acc(h * w + 1);
  REP(i, d, h * w) {
    acc[i] = acc[i - d] + abs(t[i].first - t[i - d].first)
      + abs(t[i].second - t[i - d].second);
  }
  int q;
  cin >> q;
  REP(i, 0, q) {
    int l, r;
    cin >> l >> r;
    l--, r--;
    cout << acc[r] - acc[l] << endl;
  }
}
