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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 100000;
int n;
int c[N], a[N];

int dp[N];
const int DEBUG = 0;

int main(void){
  cin >> n;
  REP(i, 0, n - 1) {
    cin >> c[i] >> a[i];
  }
  if (n >= 1000) {
    exit(1);
  }
  dp[0] = 0;
  REP(i, 1, n) {
    set<int> q;
    REP(j, 1, c[i - 1] + 1) {
      q.insert(dp[i-j]);
    }
    int r = 0;
    while (q.count(r)) { r++; }
    if (DEBUG) {
      cout << "dp[" << i << "]=" << r << endl;
    }
    dp[i] = r;
  }
  int x = 0;
  REP(i, 0, n - 1) {
    if(a[i] % 2 != 0) {
      x ^= dp[i + 1];
    }
  }
  cout << (x ? "First" : "Second") << endl;
}
