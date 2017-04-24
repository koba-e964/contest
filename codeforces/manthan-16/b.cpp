#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;

i64 calc(i64 m) {
  i64 lo = -1;
  i64 hi = m * 10;
  while(hi-lo>1){
i64 mid = (hi+lo)/2;
i64 tot=0;
i64 x=mid;
while(x>0){x/=5;tot+=x;}
if(tot>=m)hi=mid; else lo=mid;
  }
  return hi;
}

int main(void) {
int m;
cin>>m;
int st=calc(m),en=calc(m+1);
cout<<en-st<<endl;
rep(i,en-st)cout<<st+i<<(i==en-st-1?"\n":" ");
}
