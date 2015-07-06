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

const int N = 100010;
const int R = 10;
const int W = 20100 / R;

int x[N], y[N];
VI bd[W][W];

int main(void){
  int n;
  cin >> n;
  int cnt = 0;
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
    int a = x[i] / R, b = y[i] / R;
    int q = (20 + R - 1) / R;
    bool ok = 1;
    REP(j, max(-a, -q), min(W - a, q + 1)) {
      REP(k, max(-b, -q), min(W - b, q + 1)) {
	VI &v = bd[a + j][b + k];
	REP(l, 0, v.size()) {
	  if (pow(x[v[l]] - x[i], 2) + pow(y[v[l]] - y[i], 2) <= 400 - EPS) {
	    ok = 0;
	  }
	}
      }
    }
    if (ok) {
      bd[a][b].push_back(i);
      cnt++;
    }
  }
  cout << cnt << endl;
}
