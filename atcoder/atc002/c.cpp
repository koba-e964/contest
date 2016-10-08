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

const int DEBUG = 0;

int main(void){
  int n;
  cin >> n;
  VI w(n);
  VI wtot(n + 1);
  wtot[0] = 0;
  REP(i, 0, n) {
    cin >> w[i];
    wtot[i + 1] = wtot[i] + w[i];
  }
  vector<VI> p(n);
  vector<VL> c(n);
  REP(i, 0, n) {
    p[i] = VI(n - i);
    c[i] = VL(n - i);
  }

  REP(k, 0, n) {
    p[k][0] = k;
    c[k][0] = 0;
  }
  REP(s, 1, n) {
    REP(k, 0, n - s) {
      int mini = -1;
      ll ma = 1e16;
      REP(i, max(k, p[k][s - 1]), min(p[k + 1][s - 1], k + s - 1) + 1) {
	ll val = c[k][i - k] + c[i + 1][k + s - i - 1]
	  + wtot[k + s + 1] - wtot[k];
	if (ma > val) {
	  ma = val;
	  mini = i;
	}
      }
      c[k][s] = ma;
      p[k][s] = mini;
      if (DEBUG) {
	cerr << "c[" << k << "," << s << "]=" << c[k][s] << endl;
	cerr << "p[" << k << "," << s << "]=" << p[k][s] << endl;
      }
    }
  }
  cout << c[0][n - 1] << endl;
}
