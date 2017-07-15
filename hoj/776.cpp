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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, s;
  while (cin >> n >> s) {
    if (n == 0 && s == 0) {
      break;
    }
    int cnt = 0;
    REP(bits, 0, 1024) {
      if (__builtin_popcount(bits) != n) {
	continue;
      }
      int sum = 0;
      REP(i, 0, 10) {
	if (bits & 1 << i) {
	  sum += i;
	}
      }
      if (sum == s) {
	cnt += 1;
      }
    }
    cout << cnt << "\n";
  }
}
