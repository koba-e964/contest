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


const int N = 100100, B = 500, X = 1510;

int x[N],y[N], hh[N];
ll tbl[X][X] = {};
int main(void){
  int n,k;
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> x[i] >> y[i] >> hh[i];
    x[i] += B;
    y[i] += B;
  }
  REP(i, 0, k) {
    int ax, ay, w, h, d;
    cin >> ax >> ay >> w >> h >> d;
    ax += B;
    ay += B;
    tbl[ax][ay] += d;
    tbl[ax][ay + h + 1] -= d;
    tbl[ax + w + 1][ay] -= d;
    tbl[ax + w + 1][ay + h + 1] += d;
  }
  REP(i, 1, X) {
    REP(j, 0, X) {
      tbl[i][j] += tbl[i - 1][j];
    }
  }
  REP(j, 1, X) {
    REP(i, 0, X) {
      tbl[i][j] += tbl[i][j - 1];
    }
  }
  ll sum = 0;
  REP(i, 0, n) {
    sum += max(0LL, hh[i] - tbl[x[i]][y[i]]);
  }
  cout << sum << endl;
}
