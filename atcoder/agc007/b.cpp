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


int main(void){
  int n;
  cin >> n;
  VI p(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }

  p = perm_inv(p);
  REP(i, 0, n) {
    p[i] += 9e8;
  }
  REP(i, 0, n) {
    cout << 30000 * i + 1 << (i == n - 1 ? "\n" : " ");
  }
  REP(i, 0, n) {
    cout << p[i] - 30000 * i << (i == n - 1 ? "\n" : " ");
  }
}
