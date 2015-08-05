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

const int N  = 1000;
int v[N];
int dp0[N], dp1[N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> v[i];
  }
  dp0[0] = v[0];
  dp1[0] = 0;
  REP(i, 1, n) {
    dp0[i] = max(dp0[i - 1], dp1[i - 1] + v[i]);
    dp1[i] = dp0[i - 1];
  }
  cout << dp0[n - 1] << endl;
  int c = dp0[n - 1];
  VI a;
  for (int i = n - 1; i >= 0; --i) {
    if (dp0[i] == c && dp0[i] != dp1[i]) {
      a.push_back(i);
      c -= v[i];
    }
  }
  reverse(a.begin(), a.end());
  REP(i, 0, a.size()) {
    cout << a[i] + 1 << (i == a.size() - 1 ? "\n" : " ");
  }
}
