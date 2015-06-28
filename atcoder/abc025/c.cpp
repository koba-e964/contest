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

int dp[1<<19];

int b[2][3];
int c[3][2];

int rec(int board[9], int f) {
  int key = 0;
  REP(i, 0, 9) {
    key += board[i] << (2 * i);
  }
  int &ret = dp[key];
  if (ret >= 0) {
    return ret;
  }
  if (f == 0) {
    // calculates the score
    int s = 0;
    REP(i, 0, 2) {
      REP(j, 0, 3) {
	if (board[3 * i + j] == board[3 * i + j + 3]) {
	  s += b[i][j];
	}
      }
    }
    REP(i, 0, 3) {
      REP(j, 0, 2) {
	if (board[3 * i + j] == board[3 * i + j + 1]) {
	  s += c[i][j];
	}
      }
    }
    return ret = s;
  }
  if (f % 2) {
    int m = 0;
    REP(i, 0, 9) {
      if (board[i] == 0) {
	board[i] = 1;
	int sub = rec(board, f - 1);
	board[i] = 0;
	m = max(m, sub);
      }
    }
    return ret = m;
  }

  int m = 1e8;
  REP(i, 0, 9) {
    if (board[i] == 0) {
      board[i] = 2;
      int sub = rec(board, f - 1);
      board[i] = 0;
      m = min(m, sub);
    }
  }
  return ret = m;
  
}


int main(void){
  int tot = 0;
  REP(i, 0, 2) {
    REP(j, 0, 3) {
      cin >> b[i][j];
      tot += b[i][j];
    }
  }
  REP(i, 0, 3) {
    cin >> c[i][0] >> c[i][1];
    tot += c[i][0] + c[i][1];
  }
  fill_n(dp, 1 << 19, -1);
  int board[9] = {0};
  int res = rec(board, 9);
  cout << res << endl << tot - res << endl;
}
