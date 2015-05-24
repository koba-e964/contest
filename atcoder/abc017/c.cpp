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
const int N = 100010;
int n,m;

struct dat {
  int l, r, s;
  bool operator <(const dat &o) const {
    if (l != o.l) {
      return l < o.l;
    }
    if (r != o.r) {
      return r < o.r;
    }
    return s < o.s;
  }
};

dat qq[N];

int main(void){
  cin >> n >> m;
  REP(i, 0, n) {
    int l, r, s;
    cin >> l >> r >> s;
    qq[i] = (dat) {l, r, s};
  }
  sort(qq, qq + n);
  if (n >= 13) {
    return 1; // RE
  }
  int ma = 0;
  REP(bits, 0, 1 << n) {
    int en = 1; // [1,en)
    int s = 0;
    REP(i, 0, n) {
      if ((bits & (1 << i)) == 0) {
	continue;
      }
      if (en < qq[i].l) {
	break;
      }
      en = max(en, qq[i].r + 1);
    }
    if (en < m + 1) {
      REP(i, 0, n) {
	s += (bits & (1 << i)) ? qq[i].s : 0;
      }
      ma = max(ma, s);
    }
  }
  cout << ma << endl;
}
