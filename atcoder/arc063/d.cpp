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
ll a[N];

int main(void){
  int n;
  ll t;
  cin >> n >> t;
  REP(i, 0, n) {
    cin >> a[i];
  }
  // get max {a[j] - a[i] | j > i}
  ll mi = 1e16;
  VL diff(n);
  REP(i, 0, n) {
    mi = min(mi, a[i]);
    diff[i] = a[i] - mi;
  }
  ll ma = 0;
  REP(i, 0, n) {
    ma = max(ma, diff[i]);
  }
  int cnt = 0;
  REP(i, 0, n) {
    cnt += diff[i] == ma ? 1 : 0;
  }
  cout << cnt << endl;
}
