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
  int n;
  cin >> n;
  double a;
  cin >> a;
  double lo = 0;
  double hi = a + n * (n + 1) * 2;
  REP(loop_cnt, 0, 30) {
    bool ok = true;
    double b = (hi + lo) / 2;
    REP(i, 0, n) {
      double h = i * (i - n + 1) + a;
      h += i * (b - a) / (n - 1);
      if (h < 0) {
	ok = false;
      }
    }
    if (ok) {
      hi = b;
    } else {
      lo = b;
    }
  }
  printf("%.02f\n", lo);
}
