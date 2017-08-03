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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int r, c;
  cin >> r >> c;
  vector<VI> a(r, VI(c));
  REP(i, 0, r) {
    REP(j, 0, c) {
      cin >> a[i][j];
    }
  }
  int ma = 0;
  REP(bits, 0, 1 << r) {
    // x \in bits <=> row x is flipped
    int tot = 0;
    REP(j, 0, c) {
      int cnt = 0;
      REP(i, 0, r) {
	cnt += a[i][j] ^ (bits & 1 << i ? 1 : 0);
      }
      tot += max(cnt, r - cnt);
    }
    ma = max(ma, tot);
  }
  cout << ma << "\n";
}
