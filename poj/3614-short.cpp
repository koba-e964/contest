#include <algorithm>
#include <cstdio>
#define R(n)for(i=0;i<n;i++)
#define X scanf("%d%d",&x,&y)
using namespace std;typedef pair<int,int>P;P m[2510],s[2510];int c,l,x,y,t,j,i;int main(){scanf("%d%d",&c,&l);R(c)X,m[i]=P(y,x);R(l)X,s[i]=P(x,y);sort(m,m+c);sort(s,s+l);R(c)for(j=lower_bound(s,s+l,P(m[i].second,-1))-s,y=lower_bound(s,s+l,P(m[i].first,1e9))-s;j<y;j++)if(s[j].second){t++,s[j].second--;break;}printf("%d\n",t);}
