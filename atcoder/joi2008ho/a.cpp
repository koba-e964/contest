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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  int tot = 0;
  int force = -1;
  int lv = 0;
  int ff = -1;
  for (int i = n - 1; i >= 0; --i) {
    if (c[i] == force) {
      force = 1 - force;
      lv -= 1;
      if (lv <= 0) {
	ff = -1;
	force -= 1;
      }
    }
    if (i % 2 == 1) {
      force = c[i];
      lv += 1;
      if (ff == -1) {
	ff = force;
      }
    }
    tot += ff == 0 || (ff == -1 && c[i] == 0) ? 1 : 0;
  }
  cout << tot << "\n";
}
