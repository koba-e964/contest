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



int main(void){
  int n, k, x;
  cin >> n >> k >> x;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  const int W = 1024;
  VI tbl(W, 0);
  REP(i, 0, n) {
    tbl[a[i]] += 1;
  }
  REP(loop_cnt, 0, k) {
    VI tmp(W, 0);
    int cnt = 0;
    REP(i, 0, W) {
      int v = tbl[i];
      if (v % 2 == 0) {
	tmp[i] += v / 2;
	tmp[i ^ x] += v / 2;
      } else {
	tmp[i] += v / 2;
	tmp[i ^ x] += v / 2;
	if (cnt % 2 == 1) {
	  tmp[i] += 1;
	} else {
	  tmp[i ^ x] += 1;
	}
	cnt += 1;
      }
    }
    tbl = tmp;
  }
  int ma = 0;
  int mi = W;
  REP(i, 0, W) {
    if (tbl[i] > 0) {
      ma = max(ma, i);
      mi = min(mi, i);
    }
  }
  cout << ma << " " << mi << endl;
}
