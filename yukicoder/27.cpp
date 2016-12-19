#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;


int knapsack(VI &t, int val) {
  int n = t.size();
  int B = 31;
  VI dp(B, 10000);
  dp[0] = 0;
  REP(i, 0, n) {
    REP(j, 0, B) {
      if (j >= t[i]) {
	dp[j] = min(dp[j], dp[j - t[i]] + 1);
      }
    }
  }
  return dp[val];
}

int main(void){
  VI v(4);
  int mi = 0;
  REP(i, 0, 4) { cin >> v[i]; mi += v[i]; }
  REP(a, 0, 31) {
    REP(b, a + 1, 31) {
      REP(c, b + 1, 31) {
	int sum = 0;
	VI t(3);
	t[0] = a;
	t[1] = b;
	t[2] = c;
	REP(i, 0, 4) {
	  sum += knapsack(t, v[i]);
	}
	mi = min(mi, sum);
      }
    }
  }
  cout << mi << endl;
}
