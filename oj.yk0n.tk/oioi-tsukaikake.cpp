#include<iostream>
#define R(i,n)for(int i=0;i<n;i++)
using namespace std;
int n,t,u,s[100],d[101][5000];
#define c(x,y)x=max(x,y)
int main(){
  cin>>n;
  R(i,n)cin>>s[i];
  cin>>t;
  R(i,n)R(j,t)d[i][j]=-1;
  d[0][0]=0;
  R(i,n)R(j,t){u=d[i][j];if(u+1)c(d[i+1][(j+s[i])%t],u+s[i]),c(d[i+1][j],u);}
  cout<<d[n][0]<<"\n";
}
