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
const ll mod = 1e9;

const int N = 1000100;
int dp[N];

int main(void){
  int n;
  scanf("%d", &n);
  dp[0] = 1;
  REP(i, 1, N) {
    if (i % 2 == 0) {
      dp[i] += dp[i / 2];
      dp[i] %= mod;
    }
    dp[i] += dp[i - 1];
    dp[i] %= mod;
  }
  printf("%d\n", (int)dp[n]);
}
