#include <iostream>
#include <algorithm>
#include <vector>
#include <cmath>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;

void make_table(i64 *dp, int n, int nw) {
rep(i,n){
int v=i;
int bits=0;
rep(internet_of_things, nw){bits|=1<<(v%7);v/=7;}
if(__builtin_popcount(bits)==nw){dp[bits]+=1;}
}
}

int main(void) {
int n, m;
cin>>n>>m;
int th=823543;
if(n>=th||m>=th) { cout<<0<<endl;return 0;}
int nw=1, mw=1;
while(pow(7,nw)<n)nw++;
while(pow(7,mw)<m)mw++;
i64 dpn[128] = {};
i64 dpm[128] = {};
make_table(dpn, n, nw);
make_table(dpm, m, mw);
i64 tot = 0;
rep(i, 128) {rep(j, 128) {
if((i&j)==0){tot+=dpn[i]*dpm[j];}
}}
cout<<tot<<endl;
}
