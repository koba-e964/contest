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

const int SN = 600;
const int N = SN * SN;
const int B = 10;

ll h[N];
ll mat[SN][B][B];
const ll mod = 1e9 + 7;

const int DEBUG = 0;

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> h[i];
  }

  for (int i = SN - 1; i >= 0; --i) {
    REP(j, 0, B) {
      REP(k, 0, B) mat[i][j][k] = 0;
    }
    REP(j, 0, B) mat[i][j][j] = 1;
    for (int j = SN - 1; j >= 0; --j) {
      ll tmp[B] = {};
      REP(k, 0, B) {
	REP(l, 0, h[i * SN + j]) {
	  tmp[k] += mat[i][k][l];
	  tmp[k] %= mod;
	}
      }
      REP(k, 0, B - 1) REP(l, 0, B) {
	mat[i][l][B - 1 - k] = mat[i][l][B - 2 - k];
      }
      REP(k, 0, B) {
	mat[i][k][0] = tmp[k];
      }
    }
  }
  
  int d;
  cin >> d;
  REP(loop_cnt, 0, d) {
    int s, t;
    cin >> s >> t;
    s--, t--;

    ll tmp[B] = {};
    tmp[0] = 1;
    while (t > s) {
      if (t % SN == 0 && t - s >= SN) {
	ll ntmp[B] = {};
	t -= SN;
	REP(i, 0, B) {
	  REP(j, 0, B) {
	    ntmp[i] += tmp[j] * mat[t / SN][j][i];
	    ntmp[i] %= mod;
	  }
	}
	REP(i, 0, B) {
	  tmp[i] = ntmp[i];
	}
      } else {
	ll nn = 0;
	t--;
	REP(i, 0, h[t]) {
	  nn += tmp[i];
	  nn %= mod;
	}
	REP(i, 0, B - 1) {
	  tmp[B - 1 - i] = tmp[B - 2 - i];
	}
	tmp[0] = nn;
	if (DEBUG) {
	  cerr << "t=" << t << endl;
	  REP(k, 0, B) {
	    cerr << "tmp[" << k << "]=" << tmp[k] << endl;
	  }
	}
      }
    }
    cout << tmp[0] << endl;
  }
}
