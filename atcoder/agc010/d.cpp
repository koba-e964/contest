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
const int n = 17;
int dp[n][n][n];

void experiment(void) {
  REP(i, 1, n) {
    cerr << "dp[" << i << "]:" << endl;
    REP(j, 1, n) {
      cerr << "dp[" << i << "," << j << "]:";
      REP(k, 1, n) {
	ll res = 1;
	if (i + j + k == 3) {
	  dp[i][j][k] = 0;
	  continue;
	}
	int g = __gcd(i, j);
	g = __gcd(g, k);
	if (g != 1) {
	  res = 1 - dp[i / g][j / g][k / g];
	} else {
	  if (i >= 2) {
	    res &= dp[i - 1][j][k];
	  }
	  if (j >= 2) {
	    res &= dp[i][j - 1][k];
	  }
	  if (k >= 2) {
	    res &= dp[i][j][k - 1];
	  }
	}
	dp[i][j][k] = 1 - res;
	cerr << " " << dp[i][j][k];
      }
      cerr << endl;
    }
    cerr << endl;
  }
  exit(0);
}


bool winning(const VL &a) {
  int n = a.size();
  ll evenodd = 0;
  int numodd = 0;
  REP(i, 0, n) {
    evenodd += a[i] - 1;
    numodd += a[i] % 2 == 1 ? 1 : 0;
  }
  if (evenodd % 2 == 1) {
    return true;
  }
  assert (numodd >= 1);
  if (n <= 2 || numodd >= 2) {
    return false;
  }
  int p = -1;
  REP(i, 0, n) {
    if (a[i] % 2 == 1) {
      p = i;
      break;
    }
  }
  if (a[p] == 1) {
    return false;
  }
  VL b(n);
  REP(i, 0, n) {
    b[i] = a[i] - (i == p ? 1 : 0);
  }
  ll g = b[0];
  REP(i, 1, n) {
    g = __gcd(g, b[i]);
  }
  REP(i, 0, n) {
    b[i] /= g;
  }
  return not winning(b);
}

int main(void){
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  cout << (winning(a) ? "First" : "Second") << endl;
}
