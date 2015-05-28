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
const int N = 300;
short dp[N][N][N];

int g,c,p;
string s;

int score(int pos, int ty) {
  char ch = s[pos];
  int q = 0;
  switch(ch) {
  case 'C':
    q = 1; break;
  case 'G':
    q = 0; break;
  case 'P':
    q = 2; break;
  default:
    assert(0);
  }
  switch((ty - q + 3) % 3) {
  case 0:
    return 1;
  case 1:
    return 0;
  default:
    return 3;
  }
}
int main(void){
  cin >> g >> c >> p >> s;
  dp[0][0][0] = 0;
  REP (i, 0, g + 1) {
    REP (j, 0, c + 1) {
      REP (k, 0, p + 1) {
	if (i + j + k == 0) {
	  continue;
	}
	int ma = 0;
	if (i >= 1) {
	  ma = max(ma, dp[i - 1][j][k] + score(i + j + k - 1, 0));
	}
	if (j >= 1) {
	  ma = max(ma, dp[i][j - 1][k] + score(i + j + k - 1, 1));
	}
	if (k >= 1) {
	  ma = max(ma, dp[i][j][k - 1] + score(i + j + k - 1, 2));
	}
	dp[i][j][k] = ma;
      }
    }
  }
  cout << dp[g][c][p] << endl;
}
