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


int dat[11] = {0};
char c(int i) {
	switch(dat[i]) {
	case 0:
		return 'x';
	case 1:
		return '.';
	default:
		return 'o';
	}
}

int main(void){
	int a,b;
	int t;
	cin>>a>>b;
	REP(i,0,a){
		cin>>t;
		dat[t] = 1;
	}
	REP(i,0,b){
		cin>>t;
		dat[t] = 2;
	}
	printf("%c %c %c %c\n", c(7), c(8), c(9),c(0));
	printf(" %c %c %c\n", c(4), c(5), c(6));
	printf("  %c %c\n", c(2), c(3));
	printf("   %c\n", c(1));
}
