#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;



int main(void){
  int n, k;
  cin >> n >> k;
  int c = (n - 1) % (k + 1);
  cout << c << endl;
  while (1) {
    cin >> c;
    if (c >= n) {
      return 0;
    }
    cout << (c + k) / (k + 1) * (k + 1) << endl;
  }
}
