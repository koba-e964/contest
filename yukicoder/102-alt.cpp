#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
using namespace std;

int main(void){
  int x = 0;
  REP(i, 0, 4) {
    int d;
    cin >> d;
    x ^= d;
  }
  cout << (x % 4 ? "Taro" : "Jiro") << endl;
}
