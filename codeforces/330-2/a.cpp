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

int a[200][200];

int main(void){
  int n,m;
  cin >> n >> m;
  int cnt = 0;
  REP(i, 0, n) {
    REP(j, 0, 2 * m) {
      cin >> a[i][j];
    }
    REP(j, 0, m) {
      cnt += a[i][2 * j] || a[i][2 * j + 1] ? 1 : 0;
    }
  }
  cout << cnt << endl;
}
