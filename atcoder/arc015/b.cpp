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

int a[6]={0};

int main(void){
	int n;
	cin>>n;
	REP(i,0,n){
		double t1,t2;
		cin>>t1>>t2;
		if(t1>=35.0)
			a[0]++;
		if(t1>=30.0&&t1<35.0)
			a[1]++;
		if(t1>=25.0&&t1<30.0)
			a[2]++;
		if(t2>=25.0)
			a[3]++;
		if(t2<0.0&&t1>=0)
			a[4]++;
		if(t1<0)a[5]++;
	}
	REP(i,0,5){
		cout<<a[i]<<" ";
	}
	cout<<a[5]<<endl;
}
