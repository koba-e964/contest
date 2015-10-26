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
  int n, m;
  int x, y;
  cin >> n >> m >> x >> y;
  VI a(n), b(m);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, m) {
    cin >> b[i];
  }
  ll t = 0;
  int i = 0, j = 0;
  int w = 0;
  while (i < n && j < m) {
    while (i < n && t > a[i]) { ++i; } 
    if (i == n) { break; }
    t = a[i] + x;
    while (j < m && t > b[j]) { ++j; }
    if (j == m) { break; }
    t = b[j] + y;
    w++;
  }
  cout << w << endl;
}
