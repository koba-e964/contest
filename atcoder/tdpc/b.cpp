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

const int N = 1010;
VI dp[N][N];
int an, bn;

int main(void){
  cin >> an >> bn;
  VI a(an), b(bn);
  REP(i, 0, an) {
    cin >> a[i];
  }
  REP(i, 0, bn) {
    cin >> b[i];
  }
  REP(i, 0, N) {
    REP(j, 0, N) {
      VI p(2);
      p[0] = -123456;
      p[1] = -123456;
      dp[i][j] = p;
    }
  }
  REP(i, 0, an + 1) {
    REP(j, 0, bn + 1) {
      VI p(2);
      int t = (an + bn + i + j) % 2;
      if (i >= 1) {
	p = dp[i - 1][j];
	p[t] += a[an - i];
      }
      if (j >= 1) {
	VI q(2);
	q = dp[i][j - 1];
	q[t] += b[bn - j];
	if (p[t] < q[t]) {
	  p = q;
	}
      }
      dp[i][j] = p;
    }
  }
  cout << dp[an][bn][0] << endl;
}
