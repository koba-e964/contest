#include<algorithm>
#include<cstdio>
#include<queue>
#define R(i,s,n)for(i=s;i<n;i++)
typedef std::pair<int,int>P;
int n,c,f,x,y,t[1<<17],u[1<<17],o,i;P a[1<<17];
int main(void){
  scanf("%d%d%d",&n,&c,&f);n/=2;
  R(i,0,c)scanf("%d%d",&x,&y),a[i]=P(x,y);
  std::sort(a,a+c);std::priority_queue<int>p,q;
  R(i,0,c){
    y=a[i].second,p.push(y),o+=y;
    if(i>=n)o-=p.top(),p.pop();
    t[i]=o;
  }
  p=q;
  R(i,o=0,c){
    y=a[c-1-i].second,p.push(y),o+=y;
    if(i>=n)o-=p.top(),p.pop();
    u[c-i-1]=o;
  }
  y=-1;
  R(x,n,c-n)if(t[x-1]+a[x].second+u[x+1]<=f)y=x;
  printf("%d\n",y+1?a[y].first:-1);
}
