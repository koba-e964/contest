#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <iostream>
#include <cstdio>
#include <cstdlib>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int r,c;
  while(cin>>r>>c&&r&&c){
    cout<<(r%2&&c%2?"no":"yes")<<endl;
  }
}
