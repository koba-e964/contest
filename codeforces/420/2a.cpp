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



int main(void){
  int n;
  cin >> n;
  vector<VI> a(n, VI(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> a[i][j];
    }
  }
  bool ok = true;
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (a[i][j] == 1) { continue; }
      bool found = false;
      REP(k, 0, n) {
	if (i == k) { continue; }
	REP(l, 0, n) {
	  if (j == l) { continue; }
	  if (a[i][l] + a[k][j] == a[i][j]) {
	    found = true;
	  }
	}
      }
      ok &= found;
    }
  }
  cout << (ok ? "Yes" : "No") << endl;
}
