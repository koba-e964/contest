#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
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

std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    assert (0 <= p[i] && p[i] < len);
    ans[p[i]] = i;
  }
  return ans;
}

// q after p
std::vector<int> perm_comp(const std::vector<int> &q, const std::vector<int> &p) {
  int n = p.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    ret[i] = q[p[i]];
  }
  return ret;
}


int main(void){
  int n;
  cin >> n;
  VL x(n);
  VL dx(n - 1), dy(n - 1);
  REP(i, 0, n) {
    cin >> x[i];
  }
  REP(i, 0, n - 1) {
    dx[i] = x[i + 1] - x[i];
  }
  VI perm(n - 1), sum(n - 1), cur(n - 1);
  REP(i, 0, n - 1) {
    perm[i] = i;
    sum[i] = i;
  }
  int m;
  ll k;
  cin >> m >> k;
  VI a(m);
  REP(i, 0, m) {
    cin >> a[i];
    a[i]--; // a[i] = 1, ..., n - 2
    swap(perm[a[i]], perm[a[i] - 1]);
  }
  cur = perm;
  while (k > 0) {
    if (k % 2 == 1) {
      sum = perm_comp(sum, cur);
    }
    cur = perm_comp(cur, cur);
    k /= 2;
  }
  REP(i, 0, n - 1) {
    dy[i] = dx[sum[i]];
  }
  VL y(n);
  y[0] = x[0];
  REP(i, 0, n - 1) {
    y[i + 1] = dy[i] + y[i];
  }
  REP(i, 0, n) {
    cout << y[i] << endl;
  }
}
