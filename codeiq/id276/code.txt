function f(i){return((i*i*4999+8999&0xFFFF)/65536)*255}
for(r=[n=s=0];s+=11,p=256/(s-1),q=160/(7*n+6),n<16;n++)
for(i=0;i<5<<13;i++)x=i%256,y=i>>8,
a=x/p|0,
b=0|y/q,
u=x/p-a,
v=y/q-b,
t=a+b*s,
d=0|
(1-u)*(1-v)*f(t)
+u*(1-v)*f(t+1)
+(1-u)*v*f(t+s)
+u*v*f(t+1+s),
r[i]=n==0?d:0|(n+6)/(n+7)*r[i]+1/(n+7)*d;
return"256,160,"+r.join(",")