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



int main(void){
  int k;
  cin >> k;
  double dp[30] = {0};
  dp[0] = 0;
  REP(i, 1, k + 1) {
    REP(x, 1, min(7, i + 1)) {
      dp[i] += dp[i - x] / 6.0;
    }
    dp[i] += 1;
  }
  printf("%.9f\n", dp[k]);
}
