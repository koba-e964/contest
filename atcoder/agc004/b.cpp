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

const int N = 2010;
ll a[N];
ll tbl[N][N];

int main(void){
  int n;
  ll x;
  cin >> n >> x;
  REP(i, 0, n) {
    cin >> a[i];
    REP(j, 0, n) {
      tbl[i][j] = a[i];
    }
  }
  REP(i, 1, n) {
    REP(j, 0, n) {
      tbl[j][i] = min(tbl[j][i], tbl[(j + 1) % n][i - 1]);
    }
  }
  ll mi = 1e16;
  REP(i, 0, n) {
    ll sum = x * i;
    REP(j, 0, n) {
      sum += tbl[j][i];
    }
    mi = min(mi, sum);
  }
  cout << mi << endl;
}
