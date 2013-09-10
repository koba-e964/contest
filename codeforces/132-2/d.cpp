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
const double EPS=1e-9;
struct point{double x,y;point():x(0),y(0){}point(double x,double y):x(x),y(y){}
point operator-(const point& p)const{return point(x-p.x,y-p.y);}
double operator*(const point& p)const{return x*p.x+y*p.y;}
point operator+(const point& p)const{return point(x+p.x,y+p.y);}
double cross(const point& p)const{return x*p.y-y*p.x;}
};

int n,m;
const int N=100000;
ll t[N],x[N],cost[N];

int main(void){
	cin>>n>>m;
	ll sum=0;
	REP(i,0,n){
		int tt,ct;
		cin>>tt>>ct>>x[i]>>cost[i];
		t[i]=ct-tt;
		ll tmp;
		if(t[i]>0)
			tmp=min(x[i]*(ll)m+cost[i],cost[i]*((m+t[i]-1)/t[i]));
		else
			tmp=x[i]*m+cost[i];
		sum+=tmp;
	}
	cout<<sum<<endl;
}
