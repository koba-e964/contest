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

int n;

void ok(){
	cout<<"YES"<<endl;
	exit(0);
}
void fail(){
	cout<<"NO"<<endl;
	exit(0);
}

int main(void){
	cin>>n;
	int t1=0,t2=0,t4=0;
	REP(i,0,n){
		int a;
		cin>>a;
		if(a==25){
			t1++;
		}else if(a==50){
			if(t1==0)fail();
			t1--;
			t2++;
		}else{
			if(t2>=1&&t1>=1){
				t2--;
				t1--;
			}else if(t1>=3){
				t1-=3;
			}else fail();
			t4++;
		}
	}
	ok();
}
