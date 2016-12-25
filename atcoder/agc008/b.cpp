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


// I wrote this solution after reading the editorial.
int main(void){
  int n, k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL cum(n + 1), pos_left(n + 1), pos_right(n + 1);
  cum[0] = pos_left[0] = pos_right[n] = 0;
  REP(i, 0, n) {
    cum[i + 1] = cum[i] + a[i];
    pos_left[i + 1] = pos_left[i] + max(a[i], 0LL);
  }
  for (int i = n - 1; i >= 0; --i) {
    pos_right[i] = pos_right[i + 1] + max(a[i], 0LL);
  }
  ll ma = 0;
  REP(i, 0, n - k + 1) {
    ma = max(ma, pos_left[i] + max(0LL, cum[i + k] - cum[i]) + pos_right[i + k]);
  }
  cout << ma << endl;
}
