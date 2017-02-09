#include <cstdio>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;



int main(void){
  double p;
  ll n;
  cin >> p >> n;
  double sum = 1;
  double cur = 1 - 2 * p;
  while (n > 0) {
    if (n % 2 == 1) {
      sum *= cur;
    }
    cur *= cur;
    n /= 2;
  }
  printf("%.15f\n", 0.5 * (1 - sum));
}
