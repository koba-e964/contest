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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

vector<VL> mul(const vector<VL> &a, const vector<VL> &b) {
  int n = a.size();
  int m = b.size();
  int l = b[0].size();
  vector<VL> ret(n, VL(l, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	ret[i][k] = max(ret[i][k], a[i][j] + b[j][k]);
      }
    }
  }
  return ret;
}


int main(void) {
  ll n;
  int m;
  cin >> n >> m;
  vector<VL> a(m, VL(m));
  REP(i, 0, m) {
    REP(j, 0, m) {
      cin >> a[i][j];
    }
  }
  vector<VL> cur = a;
  vector<VL> sum(m, VL(m));
  n -= 1;
  while (n > 0) {
    if (n % 2 == 1) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    n /= 2;
  }
  ll ma = 0;
  REP(i, 0, m) {
    REP(j, 0, m) {
      ma = max(ma, sum[i][j]);
    }
  }
  cout << ma << endl;
}
