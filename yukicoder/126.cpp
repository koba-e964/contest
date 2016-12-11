#include <algorithm>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;


int main(){
  int a,b,s;
  cin>>a>>b>>s;
  if (a <= b) {
    if (a > 0) {
      cout << min(abs(s - b), abs(s - a)) + s << endl;
    } else {
      cout << (abs(s - b) < abs(s - a) ? abs(s - b) + s + 1 : 2 * s) << endl;
    }
  } else if (s == 1) {
    cout << abs(a - 1) + 1 << endl;
  } else {
    cout << (abs(s - b) < abs(s - a) ?
	     a + abs(s - b) + min(abs(s - a), s-1) :
	     s + abs(s - a)) << endl;
  }
} 
