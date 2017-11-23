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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<VI> c(10, VI(10));
  REP(i, 0, 10) {
    REP(j, 0, 10) {
      cin >> c[i][j];
    }
  }
  REP(k, 0, 10) {
    REP(i, 0, 10) {
      REP(j, 0, 10) {
	c[i][j] = min(c[i][j], c[i][k] + c[k][j]);
      }
    }
  }
  int tot = 0;
  REP(i, 0, h) {
    REP(j, 0, w) {
      int a;
      cin >> a;
      if (a >= 0) {
	tot += c[a][1];
      }
    }
  }
  cout << tot << "\n";
}
