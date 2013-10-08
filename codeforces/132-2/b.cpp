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

const int N=5000;
double x[N],y[N],z[N];
double r1;
int main(void){
	int n,m,k;
	double a,b;
	cin>>n;
	r1=0.0;
	REP(i,0,n){
		cin>>x[i];
		r1=max(r1,x[i]);
	}
	cin>>m;
	REP(i,0,m)cin>>y[i];
	cin>>k;
	REP(i,0,k)cin>>z[i];
	cin>>a>>b;
	double mc=0.0;
	REP(i,0,m)REP(j,0,k){
		double p1=y[i];
		double p2=z[j];
		double coef=b*p1/(a*p2+b*p1);
		mc=max(mc,coef);
	}
	printf("%.10f\n",sqrt(mc)*r1);
}

