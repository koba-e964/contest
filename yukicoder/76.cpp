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

const int N = 1e6 + 10;

const double p[7] = {
  0.0/0.0, 
  1.0/12,
  2.0/12,
  3.0/12,
  1.0/12,
  3.0/12,
  2.0/12,
};

double memo[N];

double rec(int n) {
  if (n < 0) {
    return 0;
  }
  double &ret = memo[n];
  if (ret >= 0) {
    return ret;
  }
  if (n == 0) {
    return ret = 0;
  }
  double sum = 0;
  REP(i, 1, 7) {
    sum += rec(n - i) * p[i];
  }
  return ret = sum + 1;
}

int main(void){
  int t;
  cin >> t;
  REP(i, 0, N) {
    memo[i] = -1;
  }
  REP (i, 0, N) {
    rec(i);
  }
  REP(loop_cnt, 0, t) {
    int n;
    cin >> n;
    printf("%.9f\n", rec(n));
  }
}
