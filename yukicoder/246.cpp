#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;



int main(void){
  ll a = 0, b = 2e9;
  while (b - a >= 2) {
    ll mid = (a + b) / 2;
    cout << "? " << mid << endl;
    int res;
    cin >> res;
    if (res) {
      a = mid;
    } else {
      b = mid;
    }
  }
  cout << "! " << a << endl;
}
