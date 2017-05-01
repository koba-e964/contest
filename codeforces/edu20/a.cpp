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
  vector<VI> ret(n, VI(n, 0));
  if (k > n * n) {
    cout << -1 << endl;
    return 0;
  }
  REP(i, 0, n) {
    REP(j, i, n) {
      if (k <= 0) { continue; }
      if (i == j) {
	ret[i][j] = 1;
	k -= 1;
      } else {
	if (k >= 2) {
	  ret[i][j] = 1;
	  ret[j][i] = 1;
	  k -= 2;
	}
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      cout << ret[i][j] << (j == n - 1 ? "\n" : " ");
    }
  }
}
