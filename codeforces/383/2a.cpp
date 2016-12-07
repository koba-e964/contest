#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;



int main(void){
  int n;
  cin >> n;
  if (n == 0) {
    cout << 1 << endl;
  } else {
    n = (n - 1) % 4 + 1;
    int c = 1;
    REP(i, 0, n) {
      c *= 8;
    }
    cout << c % 10 << endl;
  }
}
