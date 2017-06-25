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
typedef pair<ll, int> PLI;
const ll mod = 1e9 + 7;

ll r, c;
int n;

// Returns the relative position on the perimeter of the given rectangle.
// -1 if (x, y) is not on the perimeter.
// counter-clockwise
ll on_edge(ll x, ll y) {
  if (y == 0) {
    return x;
  }
  if (x == r) {
    return r + y;
  }
  if (y == c) {
    return r + c + r - x;
  }
  if (x == 0) {
    return 2 * (r + c) - y;
  }
  return -1;
}

int main(void){
  cin >> r >> c >> n;
  vector<PLI> pool;
  REP(i, 0, n) {
    ll x1, y1, x2, y2;
    cin >> x1 >> y1 >> x2 >> y2;
    ll pos1 = on_edge(x1, y1);
    ll pos2 = on_edge(x2, y2);
    if (pos1 >= 0 && pos2 >= 0) {
      pool.push_back(PLI(pos1, i));
      pool.push_back(PLI(pos2, i));
    }
  }
  sort(pool.begin(), pool.end());
  if (0) {
    REP(i, 0, pool.size()) {
      cerr << "pool[" << i << "]:" << pool[i].first
	   << " (" << pool[i].second << ")" << endl;
    }
  }
  vector<int> st;
  REP(i, 0, pool.size()) {
    int t = pool[i].second;
    if (st.size() == 0 || st.back() != t) {
      st.push_back(t);
    } else {
      st.pop_back();
    }
  }
  cout << (st.empty() ? "YES" : "NO") << endl;
}
