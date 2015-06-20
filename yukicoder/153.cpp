#include <iostream>
#include <set>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 110;
int dp[N];

int main(void){
  int n;
  cin >> n;
  dp[0] = 0;
  dp[1] = 0;
  REP(i, 2, N) {
    set<int> s;
    if (i % 2 == 0) {
      s.insert(0);
    } else {
      s.insert(dp[i / 2] ^ dp[i / 2 + 1]);
    }
    if (i / 3) {
      switch(i % 3) {
      case 0:
      case 2:
	s.insert(dp[i / 3]);
	break;
      case 1:
	s.insert(dp[i / 3 + 1]);
	break;
      }
    }
    int q = 0;
    while (s.count(q)) {
      q++;
    }
    dp[i] = q;
  }
  cout << (dp[n] ? 'A' : 'B') << endl;
}
