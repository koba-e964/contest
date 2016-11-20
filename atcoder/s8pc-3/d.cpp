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


const int H = 200;
int a[H][H];
ll b[H][H + 1];

int main(void){
  int h, w;
  cin >> h >> w;
  assert (h <= 3);
  REP(i, 0, h) {
    REP(j, 0, w) {
      cin >> a[i][j];
    }
  }
  ll tot = 0;
  REP(i, 0, h) {
    b[i][0] = 0;
    REP(j, 0, w) {
      tot += a[i][j];
      b[i][j + 1] = b[i][j] + a[i][j];
    }
  }
  
  if (h <= 2) {
    cout << tot << endl;
    return 0;
  }
  assert (h == 3);
  ll ma = 0;
  REP(i, 0, w) {
    REP(j, i + 1, w) {
      ll tmp = b[0][w] - b[0][j + 1];
      tmp += b[1][j] - b[1][i + 1];
      tmp += b[2][i] - b[2][0];
      ma = max(ma, tot - tmp);
    }
  }
  cout << ma << endl;
}
