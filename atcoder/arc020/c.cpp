#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int N=100000;

ll comp(ll r, ll e, ll b){ // (r^e-1)/(r-1) mod b
	if(e<=0) return 0;
	if(e%2){
		ll sub=comp(r,e-1,b);
		sub*=r;
		sub++;
		sub%=b;
		return sub;
	}
	ll sub=comp(r,e/2,b);
	ll res=sub;
	res*=sub;
	res%=b;
	res*=r-1;
	res%=b;
	res+=2*sub;
	res%=b;
	return res;
}
ll pow(ll r,ll e, ll mod){
	ll sum=1;
	ll cur=r;
	while(e>0){
		if(e%2){
			sum*=cur;
			sum%=mod;
		}
		cur*=cur;
		cur%=mod;
		e/=2;
	}
	return sum%mod;
}


int n;
ll a[N],l[N];
ll b;

ll tbl[11]={
1,
10,
100,
1000,
10000,
100000,
1000000,
10000000,
100000000,
1000000000,
10000000000LL};

int dig(ll v){
	int c=0;
	while(v>0){
		v/=10;
		c++;
	}
	return c;
}

int main(void){
	cin>>n;
	REP(i,0,n){
		cin>>a[i]>>l[i];
	}
	cin>>b;
	ll sum=0;
	REP(i,0,n){
		ll d=dig(a[i]);
		sum*=pow(10,l[i]*d,b);
		sum%=b;
		sum+=comp(tbl[d]%b,l[i],b)*a[i]%b;
		sum%=b;
	}
	cout<<sum<<endl;
	return 0;
}
