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


const int S = 12345;
const int N = 20;
int dp[S][N];

int main(void){
  int n;
  while (scanf("%d", &n) >= 1 && n) {
    int s;
    scanf("%d", &s);
    VI m(2 * n);
    REP(i, 0, 2 * n) {
      scanf("%d", &m[i]);
    }
    REP(i, 0, 2 * n) {
      dp[0][i] = 1;
    }
    REP(i, 1, s + 1) {
      REP(j, 0, 2 * n) {
	int res = 1;
	REP(k, 1, m[j] + 1) {
	  res &= i >= k ? dp[i - k][(j + 2 * n + 1) % (2 * n)] : 1;
	}
	dp[i][j] = 1 - res;
	//cerr << i << ":" << dp[i][0] << " " << dp[i][1] << endl;
      }
    }
    printf("%d\n", dp[s][0]);
  }
}
