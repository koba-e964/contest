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
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  int n, k;
  cin >> n >> k;
  int a[100] = {};
  int dp[100] = {};
  REP(i, 0, n) {
    a[i] = i;
    dp[i] = -1;
  }
  REP(i, 0, k) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    swap(a[x], a[y]);
  }
  REP(i, 0, n) {
    if (dp[i] > 0) {
      continue;
    }
    int c = 1;
    int v = a[i];
    while (v != i) {
      v = a[v];
      c++;
    }
    while (dp[v] < 0) {
      dp[v] = c;
      v = a[v];
    }
  }
  ll l = 1;
  REP(i, 0, n) {
    l = l / __gcd(l, (ll)dp[i]) * dp[i];
  }
  cout << l << endl;
}
