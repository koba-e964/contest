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

const int N = 100010;
int a[N], b[N];

int main(void){
  int n, k, x;
  cin >> n >> k >> x;
  x--;
  REP(i, 0, k) {
    if (i == x) {
      string s;
      cin >> s;
      cin >> s;
      continue;
    }
    cin >> a[i] >> b[i];
    a[i]--, b[i]--;
  }
  VI c(n);
  REP(i, 0, n) {
    cin >> c[i];
    c[i]--;
  }
  VI p(n), q(n);
  REP(i, 0, n) {
    p[i] = q[i] = i;
  }
  REP(i, 0, x) {
    swap(p[a[i]], p[b[i]]);
  }
  REP(i, x + 1, k) {
    swap(q[a[i]], q[b[i]]);
  }
  // solve p . ?? . q == c
  VI res = perm_comp(c, perm_inv(q));
  res = perm_comp(perm_inv(p), res);
  REP(i, 0, n) {
    if (res[i] > i) {
      cout << i + 1 << " " << res[i] + 1 << endl;
    }
  }
}
