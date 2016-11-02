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

const int H = 110;
int c[H][H];
int acc[H][H];

int main(void){
  int h, w;
  cin >> h >> w;
  REP(i, 0, h) {
    REP(j, 0, w) {
      cin >> c[i][j];
      if ((i + j) % 2) {
	c[i][j] *= -1;
      }
    }
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j] + c[i][j];
    }
  }
  int ma = 0;
  REP(i, 0, h) REP(j, i + 1, h + 1) {
    REP(k, 0, w) REP(l, k + 1, w + 1) {
      if (acc[j][l] - acc[j][k] - acc[i][l] + acc[i][k] == 0) {
	ma = max(ma, (j - i) * (l - k));
      }
    }
  }
  cout << ma << endl;
}
