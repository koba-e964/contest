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

int calc(VI x) {
  int n = x.size();
  int ma = 0;
  if (n == 1) {
    return x[0];
  }
  REP(i, 1, n - 1) {
    x[i] = 1 - x[i];
  }
  VI acc1(n + 1), acc2(n + 1);
  REP(i, 0, n) {
    acc1[i + 1] = acc1[i] + x[i]; 
  }
  for (int i = n - 1; i >= 0; --i) {
    acc2[i] = acc2[i + 1] + x[i];
  }
  REP(i, 0, n - 1) {
    int last;
    if (x[i] == x[i + 1]) {
      last = 1;
    } else {
      last = 2;
    }
    ma = max(ma, last + acc1[i] + acc2[i + 2]);
  }
  return ma;
}

int main(void){
  int m, n;
  cin >> m >> n;
  vector<VI> a(m, VI(n));
  REP(i, 0, m) {
    REP(j, 0, n) {
      cin >> a[i][j];
    }
  }
  int tot = 0;
  REP(i, 0, m) {
    if (i >= 1) {
      REP(j, 0, n) {
	a[i][j] = 1 - a[i][j];
      }
    }
    tot += calc(a[i]);
  }
  cout << tot << endl;
}
