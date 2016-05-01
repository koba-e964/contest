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

const int N = 51;

int tbl[N][N], d[N][N];

int ma[N * N] = {};

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> d[i][j];
      tbl[i + 1][j + 1] = d[i][j];
    }
  }

  REP(i, 0, n + 1) {
    REP(j, 0, n) {
      tbl[i][j + 1] += tbl[i][j];
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n + 1) {
      tbl[i + 1][j] += tbl[i][j];
    }
  }


  REP(j, 0, n + 1) {
    REP(i, 0, j) {
      REP(l, 0, n + 1) {
	REP(k, 0, l) {
	  int area = (l - k) * (j - i);
	  ma[area] = max(ma[area], tbl[l][j] - tbl[l][i] - tbl[k][j] + tbl[k][i]);
	}
      }
    }
  }
  REP(i, 1, N * N) {
    ma[i] = max(ma[i], ma[i - 1]);
  }
  int q;
  cin >> q;
  REP(loop_cnt, 0, q) {
    int p;
    cin >> p;
    cout << ma[p] << endl;
  }
  
}
