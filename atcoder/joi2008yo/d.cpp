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
  int m, n;
  cin >> m;
  vector<VI> a(m, VI(2));
  REP(i, 0, m) {
    cin >> a[i][0] >> a[i][1];
  }
  cin >> n;
  vector<VI> b(n, VI(2));
  set<VI> col;
  REP(i, 0, n) {
    cin >> b[i][0] >> b[i][1];
    col.insert(b[i]);
  }
  REP(i, 0, n) {
    VI diff(2);
    REP(j, 0, 2) {
      diff[j] = b[i][j] - a[0][j];
    }
    bool ok = true;
    REP(i, 0, m) {
      VI dest(2);
      REP(j, 0, 2) {
	dest[j] = diff[j] + a[i][j];
      }
      if (col.count(dest) == 0) {
	ok = false;
	break;
      }
    }
    if (ok) {
      cout << diff[0] << " " << diff[1] << "\n";
      return 0;
    }
  }
  
}
