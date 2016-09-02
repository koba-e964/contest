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
const int DEBUG = 0;

const int N = 5010;
const ll inf = 1e17;
ll dp[2][N][2][2];

int main(void){
  int n, s, e;
  cin >> n >> s >> e;
  s--, e--;
  VL x(n), a(n), b(n), c(n), d(n);
  REP(i, 0, n) {
    cin >> x[i];
  }
  REP(i, 0, n) {
    cin >> a[i];
    a[i] += x[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
    b[i] -= x[i];
  }
  REP(i, 0, n) {
    cin >> c[i];
    c[i] += x[i];
  }
  REP(i, 0, n) {
    cin >> d[i];
    d[i] -= x[i];
  }
  if (s > e) {
    swap(s, e);
    REP(i, 0, n) {
      swap(a[i], c[i]);
      swap(b[i], d[i]);
    }
  }
  b[n - 1] = d[n - 1] = inf;
  a[0] = c[0] = inf;
  c[e] = d[e] = inf;
  a[s] = b[s] = inf;
  assert (s < e);
  REP(i, 0, 2) {
    REP(j, 0, n + 1) {
      REP(k, 0, 2) {
	dp[i][j][k][0] = dp[i][j][k][1] = inf;
      }
    }
  }
  if (DEBUG) {
    cerr << "a:";
    REP(i, 0, n) {
      cerr << " " << a[i];
    }
    cerr << endl;
    cerr << "b:";
    REP(i, 0, n) {
      cerr << " " << b[i];
    }
    cerr << endl;
    cerr << "c:";
    REP(i, 0, n) {
      cerr << " " << c[i];
    }
    cerr << endl;
    cerr << "d:";
    REP(i, 0, n) {
      cerr << " " << d[i];
    }
    cerr << endl;
  }
#define MIU(a, b) (a) = min(a, b)
  dp[0][0][0][0] = 0;
  REP(i, 1, n + 1) {
    int t = i % 2;
    REP(j, 0, n + 1) {
      REP(k, 0, 2) {
	dp[t][j][k][0] = dp[t][j][k][1] = inf;
      }
    }
    REP(j, 0, n + 1) {
      REP(k, 0, 2) {
	REP(l, 0, 2) {
	  if (i != s + 1 && i != e + 1) {
	    // --->i, <---i
	    if (j >= 2 || (j >= 1 && (k >= 1 || l >= 1))) {
	      MIU(dp[t][j - 1][k][l], dp[1 - t][j][k][l] + a[i - 1] + c[i - 1]);
	    }
	    if (k == 1 && l == 1 && i == n) {
	      MIU(dp[t][j][0][0], dp[1 - t][j][k][l] + a[i - 1] + c[i - 1]);
	    }
	    // --->i, i---->
	    if (j >= 1 || k >= 1) {
	      MIU(dp[t][j][k][l], dp[1 - t][j][k][l] + a[i - 1] + d[i - 1]);
	    }
	    // i<----, <----i
	    if (j >= 1 || l >= 1) {
	      MIU(dp[t][j][k][l], dp[1 - t][j][k][l] + b[i - 1] + c[i - 1]);
	    }
	    // i<----, i---->
	    MIU(dp[t][j + 1][k][l], dp[1 - t][j][k][l] + b[i - 1] + d[i - 1]);
	  }
	  if (i == s + 1) {
	    if (k == 0) {
	      // s---->
	      MIU(dp[t][j][k + 1][l], dp[1 - t][j][k][l] + d[i - 1]);
	      // <----s
	      if (j >= 1) {
		MIU(dp[t][j - 1][k + 1][l], dp[1 - t][j][k][l] + c[i - 1]);
	      }
	    }
	  }
	  if (i == e + 1) {
	    if (l == 0) {
	      // e<-----
	      MIU(dp[t][j][k][l + 1], dp[1 - t][j][k][l] + b[i - 1]);
	      // ---->e
	      if (j >= 1) {
		MIU(dp[t][j - 1][k][l + 1], dp[1 - t][j][k][l] + a[i - 1]);
	      }
	      if (k >= 1 && e == n - 1) {
		MIU(dp[t][j][0][0], dp[1 - t][j][k][l] + a[i - 1]);
	      }
	    }
	  }
	}
      }
    }
    if (DEBUG) {
      REP(j, 0, i + 1) {
	cerr << "dp[" << i << "," << j <<"]:";
	REP(k, 0, 2) {
	  REP(l, 0, 2) {
	    if (dp[t][j][k][l] == inf) {
	      cerr << " inf";
	    } else {
	      cerr << " " << dp[t][j][k][l];
	    }
	  }
	}
	cerr << endl;
      }
    }
  }
  cout << dp[n % 2][0][0][0] << endl;
}
