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

const int N=100000;
int x[N],y[N],w[N];
int n,ww,h;
int main(void){
	cin>>n>>ww>>h;
	REP(i,0,n){
		cin>>x[i]>>y[i]>>w[i];
		if(w[i]>=ww || w[i]>=h){
			cout<<"Yes"<<endl;
			return 0;
		}
	}
	int a[N];
	int b[N];
	fill_n(a,ww,0);
	fill_n(b,h,0);
	int ac=ww,bc=h;
	REP(i,0,n){
		REP(j,x[i]-w[i],x[i]+w[i]+1){
			if(j<0 || j>=ww)continue;
			if(a[j]==0){
				a[j]=1;
				ac--;
			}
		}
		REP(j,y[i]-w[i],y[i]+w[i]+1){
			if(j<0 || j>=h)continue;
			if(b[j]==0){
				b[j]=1;
				bc--;
			}
		}
	}
	//cerr<<ac<<","<<bc<<endl;
	cout<<(ac&&bc?"No":"Yes")<<endl;
}
