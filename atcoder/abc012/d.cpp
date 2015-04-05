#include<iostream>
#include<algorithm>
 
using namespace std;
const int INF=0xa1a7a4a;
 
const int N=300;
int d[N][N];
int n,m;
#define rep(i,n) for(int i=0;i<int(n);++i)
int ch(int v){
int m=0;
rep(i,n){
m=max(m,d[v][i]);
}
return m;
}
 
int main(){
  cin >> n >> m;
  rep(i,n)
    rep(j,n)
      d[i][j] =INF;
  rep(i, n)
	d[i][i] =0;
  for(int i=0; i< m; i++){
    int x,y,a;
    cin >> x >> y >> a;
x--,y--;
   d[x][y]=d[y][x]=a;
  }
  rep(k,n)
   rep(i,n)
   rep(j,n)
   if(d[i][j] > d[i][k]+d[k][j])
(d[i][j] = d[i][k]+d[k][j]);
  int mini=-1;
  int mv=INF;
  rep(i,n)
   if(mv > ch(i))
    mv=ch(i),mini=i;
  cout<<mv<<endl;
}
