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

map<PI, int> memo;

int dfs(int n, int p) {
  p = min(p, n);
  if (memo.count(PI(n, p))) {
    return memo[PI(n, p)];
  }
  if (n == 0) {
    return 0;
  }
  REP(i, 1, p + 1) {
    int res = dfs(n - i, i + 1);
    if (res == 0) {
      return memo[PI(n, p)] = 1;
    }
  }
  return memo[PI(n, p)] = 0;
}

int main(void){
  int n, p;
  cin >> n >> p;
  cout << (dfs(n, p) ? "first" : "second") << endl;
}
