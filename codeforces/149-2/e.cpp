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

const int N=1<<17;
int a[N];
ll bit[N];



int main(void){
	int m;
	cin>>n;
	REP(i,0,n)cin>>a[i];
	fill_n(bit,N,0);
	cin>>>m;
	REP(i,0,m){
		int t;
		cin>>t;
		if(t==1){
			int l,r;
			cin>>l>>r;
		}else{//t==2
			int l,r,x;
			cin>>l>>r>>x;
		}
	}
}
