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

const int M = 10;
const int N = 1 << M;
double dp[N][M];
double s[N];

int m;

double rec(int bits, int lv) {
  double &ret = dp[bits][lv];
  if (ret >= 0) {
    return ret;
  }
  if (lv == 0) {
    return ret = 1;
  }
  int mask = (1 << lv - 1) - 1;
  int start = (bits ^ (1 << lv - 1)) & ~mask;
  double total = 0.0;
  REP(i, 0, 1 << lv - 1) {
    double sub = rec(start + i, lv - 1);
    double odds = s[bits] * s[bits];
    odds = odds / (odds + s[start + i] * s[start + i]);
    total += odds * sub;
  }
  return ret = total * rec(bits, lv - 1);
}

int main(void){
  cin >> m;
  REP(i, 0, 1 << m) {
    cin >> s[i];
  }
  REP(i, 0, N) {
    REP(j, 0, M) {
      dp[i][j] = -1;
    }
  }
  printf("%.9f\n", rec(0, m));
}
