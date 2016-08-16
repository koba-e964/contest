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
  int n;
  cin >> n;
  vector<PI> iv;
  REP(i, 0, n) {
    int x, r;
    cin >> x >> r;
    iv.push_back(PI(x - r, - (x + r)));
  }
  VI dp(n + 1);
  int l = 0;
  sort(iv.begin(), iv.end());
  // longest increasing subsequence
  vector<PI> pool;
  REP(i, 0, n + 1) {
    if (i == n || (i >= 1 && iv[i].first > iv[i - 1].first)) {
      REP(j, 0, pool.size()) {
	int newL = pool[j].first;
	int ni = pool[j].second;
	if (newL > l) {
	  l = newL;
	  dp[newL] = ni;
	} else {
	  if (iv[dp[newL]].second > iv[ni].second) {
	    dp[newL] = ni;
	  }
	}
      }
      pool.clear();
    }
    if (i == n) {
      break;
    }
	
    int lo = 0, hi = l + 1;
    while (hi - lo > 1) {
      int mid = (lo + hi) / 2;
      if (iv[dp[mid]].second < iv[i].second) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    int newL = lo + 1;
    pool.push_back(PI(newL, i));
  }
  cout << l << endl;
}
