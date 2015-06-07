#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

int dp[400];

int main(void){
  ll n;
  cin >> n;
  REP (i, 0, 400) {
    dp[i] = (i + i / 4 - i / 100 + i / 400) % 7;
  }
  ll c = 0;
  ll q = 0; 
  REP(i, 15, n % 400 + 401) {
    c += dp[14] == dp[i % 400];
  }
  REP(i, 0, 400) {
    q += dp[14] == dp[i];
  }
  cout << c + q * (n / 400 - 6) << endl;
}
