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
const int MOD=777777777;
const int N=78000;
int n,m;
int good[4][4];
int v[N],t[N];
int main(void){
	cin>>n>>m;
	fill_n((int*)good,4*4,0);
	REP(i,1,4){
		REP(j,1,4)
			cin>>good[i][j];
	}
	REP(i,0,m)
		cin>>v[i]>>t[i];
	
}
