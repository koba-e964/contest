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


int main(void){
  int n, k;
  cin >> n >> k;
  int seq[4] = {1,2,0,3};
  VI ans(n);
  REP(i, 0, n - k) {
    ans[i] = seq[i % 4];
  }
  REP(i, n - k, n) {
    ans[i] = ans[n - k - 1];
  }
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
