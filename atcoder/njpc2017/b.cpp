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
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;



int main(void){
  int h, w, n;
  cin >> h >> w >> n;
  ll tot = (ll) (h - 1) * w + (ll) h * (w - 1);
  set<PL> nc;
  REP(i, 0, n) {
    ll r, c;
    cin >> r >> c;
    int dx[4] = {1, 0, -1, 0};
    int dy[4] = {0, 1, 0, -1};
    REP(d, 0, 4) {
      int nx = r + dx[d];
      int ny = c + dy[d];
      if (nx >= 1 && nx <= h && ny >= 1 && ny <= w) {
	ll p1 = r * w + c;
	ll p2 = nx * w + ny;
	if (p1 > p2) {
	  swap(p1, p2);
	}
	assert (p1 != p2);
	nc.insert(PL(p1, p2));
      }
    }
  }
  cout << tot - nc.size() << endl;
}
