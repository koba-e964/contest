a[1<<17];x;z;r(t){return a[t]=a[t]-t?r(a[t]):t;}main(y){for(gets(a),x=1<<17;x--;a[x]=x);for(;~scanf("%d%d%d",&x,&y,&z);x?puts(r(y)-r(z)?"NO":"YES"):(a[r(y)]=r(z)));return 0;}
