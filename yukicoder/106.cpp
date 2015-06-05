#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 2001000;
int dp[N];

int main(void){
  int n,k;
  cin >> n >> k;  
  REP (i, 2, N) {
    if (dp[i]) {
      continue;
    }
    for (int j = i; j < N; j += i) {
      dp[j]++;
    }
  }
  int cnt = 0;
  REP(i, 2, n + 1) {
    cnt += dp[i] >= k;
  }
  cout << cnt << endl;
}
