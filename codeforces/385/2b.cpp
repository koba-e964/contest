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



int main(void){
  int n, m;
  cin >> n >> m;
  vector<string> b(n);
  REP(i, 0, n) {
    cin >> b[i];
  }
  int mix = 1000, ma_x = -1, miy = 1000, ma_y = -1;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (b[i][j] == 'X') {
	mix = min(mix, i);
	ma_x = max(ma_x, i);
	miy = min(miy, j);
	ma_y = max(ma_y, j);
      }
    }
  }
  bool ok = true;
  REP(i, mix, ma_x + 1) {
    REP(j, miy, ma_y + 1) {
      ok &= b[i][j] == 'X';
    }
  }
  cout << (ok ? "YES" : "NO") << endl;
}
