#define R(n)for(i=0;i<n;i++)
h[210],e[210],i,n,m,s,r,g[210][210],l[210];d(v){int i;e[v]=1;R(l[v]){int u=g[v][i],w=h[u];if(w<0||(!e[w]&&d(w)))return h[u]=v,1;}return 0;}main(){for(;scanf("%d%d",&n,&m)>1;printf("%d\n",r)){R(n)for(l[i]=r=0,h[i]=-scanf("%d",&s);s--;g[i][l[i]++]=m-1)scanf("%d",&m);R(n)memset(e,0,840),r+=d(i);}}
