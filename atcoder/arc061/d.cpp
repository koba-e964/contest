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


int h, w, n;
vector<PI> ptv;
set<PI> pts;
set<PI> nbr;
int main(void){
  cin >> h >> w >> n;
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    ptv.push_back(PI(a, b));
    pts.insert(PI(a, b));
  }
  ll a[10] = {0};
  REP(i, 0, ptv.size()) {
    PI cur = ptv[i];
    int cnt = 0;
    REP(dx, -1, 2) {
      REP(dy, -1, 2) {
	PI t(cur.first + dx, cur.second + dy);
	if (t.first >= 2 && t.first <= h - 1 && t.second >= 2 && t.second <= w - 1) {
	  nbr.insert(t);
	}
      }
    }
  }
  for (set<PI>::iterator it = nbr.begin(); it != nbr.end(); ++it) {
    PI cur = *it;
    int cnt = 0;
    REP(dx, -1, 2) {
      REP(dy, -1, 2) {
	PI t(cur.first + dx, cur.second + dy);
	if (pts.count(t)) {
	  cnt++;
	}
      }
    }
    assert (cnt >= 1 && cnt <= 9);
    a[cnt]++;
  }
  a[0] = (ll)(h - 2) * (w - 2);
  REP(i, 1, 10) {
    a[0] -= a[i];
  }
  REP(i, 0, 10) {
    cout << a[i] << endl;
  }
}
