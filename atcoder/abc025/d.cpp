#include <iostream>
#include <cstring>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;
const ll mod = 1e9 + 7;

int gl_used = 0;
int dp[1 << 25];

int x[5][5];

int rec(int used, int p) {
  int &ret = dp[used];
  if (p >= 25) {
    return ret = 1;
  }
  if (ret >= 0) {
    return ret;
  }
  int target = -1;
  if (gl_used & (1 << p)) {
    target = p;
  }
  ll sum = 0;
  REP(i, 0, 5) {
    REP(j, 0, 5) {
      int q = 5 * i + j;
      if (used & (1 << q)) {
	continue;
      }
      if (x[i][j] == target) {
	bool cond[2] = {i >= 1 && i <= 3, j >= 1 && j <= 3};
	int dist[4] = {-5, 5, -1, 1};
	bool ok = 1;
	REP(a, 0, 4) {
	  if (cond[a / 2]
	      && (used & (1 << (q + dist[a])))
	      && !(used & (1 << (q - dist[a])))) {
	    ok = 0;
	    break;
	  }
	}
	if (ok) {
	  sum += (ll)rec(used | (1 << q), p + 1);
	}
	sum %= mod;
      }
    }
  }
  return ret = sum;
}

int main(void){
  REP(i, 0, 5) {
    REP(j, 0, 5) {
      cin >> x[i][j];
      x[i][j]--;
      if (x[i][j] >= 0) {
	gl_used |= 1 << x[i][j];
      }
    }
  }
  memset(dp, -1, sizeof dp);
  int r = rec(0, 0);
  cout << r << endl;
}
