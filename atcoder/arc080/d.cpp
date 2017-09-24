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
  int h, w, n;
  cin >> h >> w >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VI dat(h * w);
  int pos = 0;
  REP(i, 0, n) {
    REP(j, 0, a[i]) {
      dat[pos] = i + 1;
      pos += 1;
    }
  }
  vector<VI> res(h, VI(w, -1));
  pos = 0;
  REP(i, 0, h) {
    int dir = i % 2 ? -1 : 1;
    int start = i % 2 ? w - 1: 0;
    int end = i % 2 ? -1 : w;
    for (int j = start; j != end; j += dir) {
      res[i][j] = dat[pos];
      pos += 1;
    }
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      cout << res[i][j] << (j == w - 1 ? "\n" : " ");
    }
  }
}
