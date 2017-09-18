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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  int tot = 0;
  REP(i, 0, n) {
    cin >> a[i];
    tot += a[i];
  }
  vector<VI> dp(n + 1, VI(2)); // Alice, b = 0: Alice, b = 1: Bob
  dp[n][0] = dp[n][1] = 0;
  for (int i = n - 1; i >= 0; --i) {
    int atake = dp[i + 1][1] + a[i];
    int btake = dp[i + 1][0];
    dp[i][0] = max(atake, btake);
    dp[i][1] = min(atake, btake);
  }
  cout << dp[0][1] << " " << tot - dp[0][1] << endl;
}
