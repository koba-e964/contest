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
    int d;
    cin >> d;
    if (d >= n) {
      return 0;
    }
    c += k + 1;
    cout << c << endl;
  }
}
