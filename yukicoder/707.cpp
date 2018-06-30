#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<string> p(h);
  REP(i, 0, h) cin >> p[i];
  vector<PI> cand;
  REP(i, 0, h) {
    cand.push_back(PI(i, -1));
    cand.push_back(PI(i, w));
  }
  REP(i, 0, w) {
    cand.push_back(PI(-1, i));
    cand.push_back(PI(h, i));
  }
  double mi = 1e18;
  for (PI c: cand) {
    double sum = 0;
    int x = c.first, y = c.second;
    REP(i, 0, h) {
      REP(j, 0, w) {
        if (p[i][j] == '1') {
          double sqDist = pow(x - i, 2) + pow(y - j, 2);
          sum += sqrt(sqDist);
        }
      }
    }
    mi = min(mi, sum);
  }
  cout << setprecision(20) << mi << endl;
}
