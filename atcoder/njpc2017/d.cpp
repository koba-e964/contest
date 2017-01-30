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

VI get_perm(int n, int &inv) {
  VI ret(n);
  REP(i, 0, n) {
    if (inv >= n - i - 1) {
      ret[i] = n - i - 1;
      inv -= n - i - 1;
      continue;
    }
    int q = inv;
    inv = 0;
    ret[i] = q;
    REP(j, 0, q) {
      ret[i + j + 1] = j;
    }
    REP(j, q + 1, n - i) {
      ret[i + j] = j;
    }
    return ret;
  }
  return ret;
}

int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  // permutation of rows
  int row_inv = min(k / m, n * (n - 1) / 2);
  int rem = k - row_inv * m;
  VI row_perm = get_perm(n, row_inv);
  REP(i, 0, n) {
    VI col = get_perm(m, rem);
    REP(j, 0, m) {
      cout << row_perm[i] * m + col[j] + 1 << (j == m - 1 ? "\n" : " ");
    }
  }
}
