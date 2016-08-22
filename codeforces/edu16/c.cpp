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

const int N = 50;
int tbl[N][N];

int main(void){
  int n;
  cin >> n;
  int c = 1, d = 2;
  REP(i, 0, n) {
    REP(j, 0, n) {
      int t = abs(i - n / 2) + abs(j - n / 2);
      if (t <= n / 2) {
	tbl[i][j] = c;
	c += 2;
      } else {
	tbl[i][j] = d;
	d += 2;
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      cout << tbl[i][j] << (j == n - 1 ? "\n" : " ");
    }
  }
}
