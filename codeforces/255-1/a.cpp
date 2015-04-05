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

const int N=200000;

#define DEBUG 0

int n;
int a[N];

int dp0[N+1], dp1[N+1]; // [dp0[i]..i] is increasing

int main(void){
	cin>>n;
	REP(i,0,n) {
		cin >> a[i];
		a[i] -= i;
	}
	dp0[0]=0;
	dp1[0]=0;
	REP(i,1,n){
		if(a[i] < a[i-1]) {
			dp0[i] = i;
			dp1[i] = dp0[i-1];
		} else {
			dp0[i] = dp0[i-1];
			if(i >= 2 && a[i-2] > a[i-1] && a[i-2] > a[i]) {
				dp1[i] = i-2;
			} else {
				dp1[i] = dp1[i-1];
			}
		}
	}
	int m = 0;
	REP(i, 0, n) {
		m = max(m, i-dp1[i]+1);
	}
	if(DEBUG) {
		REP(i,0,n) {
			cout << dp0[i] << " ";
		}
		cout << endl;
		REP(i,0,n) {
			cout << dp1[i] << " ";
		}
		cout << endl;
	}
	cout << m<< endl;
}
