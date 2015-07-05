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

const int N = 501;
int b[N][N];
int a[N][N];

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    string s;
    cin >> s;
    REP(j, 0, m) {
      b[i][j] = s[j] - '0';
    }
  }
  int dx[4] = {1,0,-1,0}, dy[4] = {0,1,0,-1};
  REP(i, 1, n - 1) {
    REP(j, 1, m - 1) {
      a[i][j] = b[i - 1][j];
      REP(d, 0, 4) {
	b[i + dx[d]][j + dy[d]] -= a[i][j];
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, m) {
      cout << a[i][j];
    }
    cout << endl;
  }
}
