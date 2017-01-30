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

const int DEBUG = 0;

void fail(void) {
  cout << "NO" << endl;
  exit(0);
}

int main(void){
  int n;
  cin >> n;
  ll l;
  cin >> l;
  VL x(n);
  REP(i, 0, n) {
    cin >> x[i];
    x[i] *= 2;
  }
  REP(i, 0, n - 1) {
    if (x[i + 1] - x[i] == 2 * l) {
      fail();
    }
  }
  ll p = 1;
  int cur = 0;

  while (cur < n) {
    int idx = cur;
    while (idx < n - 1 && x[idx + 1] - x[idx] < 2 * l) {
      idx++;
    }
    if (x[idx] - x[cur] >= 2 * l) {
      fail();
    }
    ll leap = max(p, x[idx] + 1 - 2 * l);
    if (idx < n - 1 && x[idx + 1] < leap + 4 * l) {
      fail();
    }
    p = leap + 4 * l;
    cur = idx + 1;
  }
  cout << "YES" << endl;
}
