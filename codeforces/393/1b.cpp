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

const int N = 100100;

ll dp[N];

int main(void){
  int n;
  cin >> n;
  VL t(n);
  REP(i, 0, n) {
    cin >> t[i];
  }
  dp[0] = 0;
  REP(i, 0, n) {
    ll mi = dp[i] + 20;
    int idx1 = upper_bound(t.begin(), t.begin() + i, t[i] - 90) - t.begin();
    mi = min(mi, dp[idx1] + 50);
    int idx2 = upper_bound(t.begin(), t.begin() + i, t[i] - 1440) - t.begin();
    mi = min(mi, dp[idx2] + 120);
    dp[i + 1] = mi;
  }
  REP(i, 0, n) {
    cout << dp[i + 1] - dp[i] << endl;
  }
}
