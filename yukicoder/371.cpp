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

const int N = 105000;
const int M = 1e6;

int main(void){
  ll l, h;
  cin >> l >> h;
  VI a(N);
  VI b(M);
  REP(i, 2, N) a[i] = 1;
  REP(i, 2, N) {
    REP(j, 2, N / i) {
      a[i * j] = 0;
    }
  }
  // a: prime
  if (h - l >= M) {
    l = h - M + 1;
  }
  REP(i, 2, N) {
    if (a[i]) {
      for (ll j = max((l + i - 1) / i * i, 2LL * i); j <= h; j += i) {
	if (b[j - l] == 0) b[j - l] = i;
      }
    }
  }
  int ma = 0;
  for (ll i = 0; i < h - l + 1; ++i) {
    if (b[i] >= b[ma]) ma = i;
  }
  cout << ma + l << endl;
  return 0;
}
