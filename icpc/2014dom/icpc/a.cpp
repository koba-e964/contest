#include <iostream>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

#define rep(i, n)       reps(i, 0, n)
#define reps(i, m, n)   for (int i = m; i < int(n); ++i)

int main()
{
    int a,b,s;
    
    while(cin>>a>>b>>s&&a!=0){
        int ans=0;
        reps(i,1,s){
            reps(j,1,s){
                int x=i*(100+a)/100;
                int y=j*(100+a)/100;
                int x2=i*(100+b)/100;
                int y2=j*(100+b)/100;
                if(x+y==s){
                   ans=max(ans,x2+y2); 
                }
            }
        }
        cout<<ans<<endl; 
    }
}

