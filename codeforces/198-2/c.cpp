#include <algorithm>
#include <cassert>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;
struct point{double x,y;point():x(0),y(0){}point(double x,double y):x(x),y(y){}
point operator-(const point& p)const{return point(x-p.x,y-p.y);}
double operator*(const point& p)const{return x*p.x+y*p.y;}
point operator+(const point& p)const{return point(x+p.x,y+p.y);}
double cross(const point& p)const{return x*p.y-y*p.x;}
};

ll gcd(ll x,ll y){
	while(y!=0){
		ll r=x%y;
		x=y;y=r;
	}return x;
}

struct frac{ll x,y;frac(ll x,ll y):x(x),y(y){
	reduce();
}
frac operator+(const frac& f)const{
	ll g=gcd(y,f.y);
	return frac(x*(f.y/g)+f.x*(y/g),y/g*f.y);
}
frac operator*(const frac& f)const{
	return frac(x*f.x,y*f.y);
}
void reduce(){
	ll g=gcd(x,y);
	x/=g;
	y/=g;
	if(y<0){
		x=-x;y=-y;
	}
}
};

const int N=100000;
int a[N];
int n;
int main(void){
	cin>>n;
	REP(i,0,n)cin>>a[i];
	sort(a,a+n);
	frac sum(0,1);
	REP(y,1,n+1){
		sum=sum+frac(4*y-1-2*n,n)*frac(a[y-1],1);
	}
	sum.reduce();
	cout<<sum.x<<" "<<sum.y<<endl;
}
