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

const int N = 2000;
int c[N][N];

int main(void){
  int t;
  cin >> t;
  c[0][0] = 1;
  REP(i, 1, N) {
    c[i][0] = 1;
    REP(j, 0, i + 1) {
      c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % 9;
    }
  }
  while (t--) {
    string s;
    cin >> s;
    int tot = 0;
    int n = s.length();
    bool all_zero = 1;
    REP(i, 0, n) {
      int u = s[i] - '0';
      if (u != 0) {
	all_zero = false;
      }
      tot += u * c[n - 1][i];
      tot %= 9;
    }
    cout << (all_zero ? 0 : 1 + (tot + 8) % 9) << endl;
  }
}
