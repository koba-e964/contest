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

int dig(int v) {
  int d = 0;
  while (v > 0) {
    d |= 1 << (v % 10);
    v /= 10;
  }
  return d;
}

int main(void){
  int n, k;
  cin >> n >> k;
  int d = 0;
  REP(i, 0, k) {
    int t; cin >> t;
    d |= 1 << t;
  }
  REP(i, n, 10 * n) {
    if ((dig(i) & d) == 0) {
      cout << i << endl;
      return 0;
    }
  }
  assert (0);
}
