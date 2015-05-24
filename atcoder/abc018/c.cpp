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

int r,c,k;
const int R = 510;

string s[R];
int rg[R][R];
int down[R][R];

int main(void){
  cin >> r >> c >> k;
  REP(i, 0, r) {
    cin >> s[i];
  }
  REP(i, 0, r) {
    int acc = 0;
    for (int j = c - 1; j >= 0; j--) {
      if (s[i][j] == 'o') {
	acc ++;
      } else {
	acc = 0;
      }
      rg[i][j] = acc;
    }
  }
  int sum = 0;
  REP(i, k - 1, r - k + 1) {
    REP(j, k - 1, c - k + 1) {
      int ok = 1;
      REP(e, 0, k) {
	ok &= rg[i - k + 1 + e][j - e] >= 2 * e + 1;
      }
      REP(e, 0, k - 1) {
	ok &= rg[i + k - 1 - e][j - e] >= 2 * e + 1;
      }
      sum += ok;
    }
  }
  cout << sum << endl;
}
