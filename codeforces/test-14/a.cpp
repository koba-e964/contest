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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, d;
  cin >> n >> d;
  string s;
  cin >> s;
  int cnt = 0;
  int pos = 0;
  while (pos < n - 1) {
    int ma = -1;
    REP(i, 1, d + 1) {
      if (s[pos + i] == '1') {
	ma = pos + i;
      }
    }
    if (ma < 0) {
      cout << -1 << endl;
      return 0;
    }
    pos = ma;
    cnt += 1;
  }
  cout << cnt << endl;
}
