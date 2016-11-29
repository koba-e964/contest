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


int tbl[101][100];

void solve(int x, int len) {
  if (len % 2 == 1) {
    REP(i, 0, len) {
      REP(j, 0, len - 1) {
	tbl[x + i][j] = x + (i + j + 1) % len;
      }
    }
    return;
  }
  if (len == 4) {
    int magic[4][3] = {
      {1,2,3},
      {3,0,2},
      {1,3,0},
      {2,0,1}};
    REP(i, 0, 4) {
      REP(j, 0, 3) {
	tbl[x + i][j] = x + magic[i][j];
      }
    }
    return;
  }
  assert (len % 2 == 0);
  solve(x, len / 2);
  solve(x + len / 2, len / 2);
  REP(j, 0, len / 2) {
    REP(i, 0, len / 2) {
      tbl[x + i][len / 2 - 1 +j] = x + len / 2 + (i + j) % (len / 2);
      tbl[x + len / 2 + i][len / 2 - 1 + j] = x + (i - j + len / 2 + 1) % (len / 2);
    }
  }
}


int main(void){
  int n;
  cin >> n;
  if (n == 2) {
    cout << -1 << endl;
    return 0;
  }
  solve(0, n);
  REP(i, 0, n) {
    REP(j, 0, n - 1)  {
      cout << tbl[i][j] + 1 << (j == n - 2 ? "\n" : " ");
    }
  }
}
