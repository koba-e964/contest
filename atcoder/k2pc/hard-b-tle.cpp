#include <stdio.h>
#include <stdlib.h>
#define TEST 0
typedef long long int i64;
const int N=100000;
struct Dat{int p;i64 q;};
int comp(const void*a,const void* b)
{
	Dat *da=(Dat*)a,*db=(Dat*)b;
	if(da->p!=db->p)
		return da->p-db->p;
	return (da->q-db->q > 0)?1:-1;
}
bool par(Dat *a,Dat *b)//a is b's child
{
	if(a->p<=b->p)return false;
	i64 mask=(1LL<<a->p)-(1LL<<(a->p-b->p));
	if(TEST)printf("mask=%x",mask);
	return ((a->q^(b->q<<(a->p-b->p)))&mask)==0;
}
int main(void)
{
	int k,n;
	Dat dat[N];
	scanf("%d%d",&k,&n);
	for(int i=0;i<n;i++)
	{
		scanf("%d%lld",&dat[i].p,&dat[i].q);
		dat[i].q--;
	}
	qsort(dat,n,sizeof(Dat),comp);
#if TEST
	for(int i=0;i<n;i++)
	{
		printf("(%d,%lld)\n",dat[i].p,dat[i].q);
	}
#endif
	i64 cnt=(1LL<<(k+1))-1;
	for(int i=0;i<n;i++)
	{
		bool hasP=false;
		for(int j=0;j<i&& !hasP;j++)
		{
			hasP=par(dat+i,dat+j);
		}
		if(hasP)continue;
		cnt-=(1LL<<(k+1-dat[i].p))-1;
	}
	printf("%lld\n",cnt);
}