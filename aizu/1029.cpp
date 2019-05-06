#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int main(void) {
  int n,m;
  for(;cin>>n>>m&&n+m;){
    VI a(n+m+1);
    REP(i,0,n+m)cin>>a[i];
    sort(a.begin(),a.end());
    int k=0;
    REP(i,0,a.size()-1)k=max(k,a[i+1]-a[i]);
    cout<<k<<"\n";
  }
}
