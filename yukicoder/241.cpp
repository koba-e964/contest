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

int a[50];
int sol[50];

bool solve(ll bits, int pop, int v) {
  if (pop == 0) {
    return 1;
  }
  int r = rand() % pop;
  ll rem = bits;
  REP(i, 0, r) {
    rem &= rem - 1;
  }
  rem = rem & -rem;
  int l = log(rem)/ log(2) + 0.4;
  if (a[v] == l) {
    return 0;
  }
  sol[v] = l;
  return solve(bits ^ rem, pop - 1, v + 1);
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  srand(10875);
  REP(i, 0, 2000) {
    if (solve((1LL << n) - 1, n, 0)) {
      REP(i, 0, n) {
	cout << sol[i] << endl;
      }
      return 0;
    }
  }
  cout << -1 << endl;
}
